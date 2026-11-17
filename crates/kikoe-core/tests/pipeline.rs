//! Pipeline behaviour tests with deterministic stand-in engines.
//!
//! These are SIMULATIONS of the threading/queueing/switching logic — they do not
//! exercise Whisper, llama.cpp or a sound card (real-engine runs are recorded in
//! EVIDENCE.md). The stand-ins live only in this test file.

use std::collections::VecDeque;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use kikoe_core::audio::{AudioFormat, AudioPacket, CaptureError, CaptureStats, CaptureStream, SourceInfo, SourceKind};
use kikoe_core::clock::PipelineClock;
use kikoe_core::lang::{LanguageControl, TargetLanguage};
use kikoe_core::pipeline::*;
use kikoe_core::segmenter::Vad;
use kikoe_core::translate::{PromptStyle, RenderedPrompt};

/// 16 kHz mono source: speech = 0.5 amplitude, silence = 0. Real-time paced.
struct ScriptCapture {
    info: SourceInfo,
    samples: VecDeque<f32>,
    clock: PipelineClock,
    seq: u64,
    start: Option<Instant>,
    released: usize,
    stopped: Arc<AtomicBool>,
}

impl ScriptCapture {
    /// `script`: (speech seconds, silence seconds) pairs.
    fn new(script: &[(f32, f32)], clock: PipelineClock) -> (Self, Arc<AtomicBool>) {
        let mut s = VecDeque::new();
        for (sp, si) in script {
            s.extend(std::iter::repeat_n(0.5f32, (sp * 16_000.0) as usize));
            s.extend(std::iter::repeat_n(0.0f32, (si * 16_000.0) as usize));
        }
        let stopped = Arc::new(AtomicBool::new(false));
        (
            Self {
                info: SourceInfo { id: "script".into(), label: "script".into(), kind: SourceKind::File, is_default: false },
                samples: s,
                clock,
                seq: 0,
                start: None,
                released: 0,
                stopped: stopped.clone(),
            },
            stopped,
        )
    }
}

impl CaptureStream for ScriptCapture {
    fn source(&self) -> &SourceInfo {
        &self.info
    }
    fn next_packet(&mut self, timeout: Duration) -> Result<Option<AudioPacket>, CaptureError> {
        if self.samples.is_empty() {
            return Ok(None);
        }
        let start = *self.start.get_or_insert_with(Instant::now);
        let due = start + Duration::from_secs_f64((self.released + 160) as f64 / 16_000.0);
        let now = Instant::now();
        if due > now {
            if due - now > timeout {
                std::thread::sleep(timeout);
                return Ok(None);
            }
            std::thread::sleep(due - now);
        }
        let n = 160.min(self.samples.len());
        let samples: Vec<f32> = self.samples.drain(..n).collect();
        self.released += n;
        self.seq += 1;
        Ok(Some(AudioPacket { seq: self.seq, format: AudioFormat { sample_rate: 16_000, channels: 1 }, samples, captured_at_ns: self.clock.now_ns(), discontinuity: false }))
    }
    fn is_finished(&self) -> bool {
        self.samples.is_empty()
    }
    fn stats(&self) -> CaptureStats {
        CaptureStats::default()
    }
    fn stop(&mut self) {
        self.stopped.store(true, Ordering::Release);
        self.samples.clear();
    }
}

struct LevelVad;
impl Vad for LevelVad {
    fn frame_probability(&mut self, frame: &[f32]) -> f32 {
        if kikoe_core::audio::rms(frame) > 0.1 { 0.95 } else { 0.02 }
    }
    fn reset(&mut self) {}
    fn name(&self) -> &str {
        "level"
    }
}

struct FakeAsr {
    delay: Duration,
    n: u32,
}
impl AsrEngine for FakeAsr {
    fn transcribe(&mut self, _samples: &[f32], abort: &Arc<AtomicBool>) -> Result<AsrOutput, String> {
        let t0 = Instant::now();
        while t0.elapsed() < self.delay {
            if abort.load(Ordering::Relaxed) {
                return Err("aborted".into());
            }
            std::thread::sleep(Duration::from_millis(5));
        }
        self.n += 1;
        Ok(AsrOutput { text: format!("発話その{}です", self.n), no_speech_prob: 0.01, avg_token_prob: 0.9, tokens: 5 })
    }
    fn describe(&self) -> String {
        "fake-asr".into()
    }
}

struct FakeMt {
    delay: Duration,
    lang: Option<TargetLanguage>,
    die_on_call: Option<u32>,
    calls: Arc<AtomicU32>,
}
impl TranslationEngine for FakeMt {
    fn prepare(&mut self, lang: TargetLanguage) -> Result<Option<u64>, String> {
        self.lang = Some(lang);
        Ok(Some(1))
    }
    fn prompt_style(&self, _: TargetLanguage) -> PromptStyle {
        PromptStyle::ChatMlNoThink
    }
    fn translate(&mut self, prompt: &RenderedPrompt, _stop: &[String], cancel: &CancelFlags<'_>) -> Result<TranslationOutput, TranslateError> {
        let call = self.calls.fetch_add(1, Ordering::AcqRel) + 1;
        if self.die_on_call == Some(call) {
            self.lang = None;
            return Err(TranslateError::WorkerDied("simulated crash".into()));
        }
        let t0 = Instant::now();
        while t0.elapsed() < self.delay {
            if cancel.is_set() {
                return Err(TranslateError::Cancelled);
            }
            std::thread::sleep(Duration::from_millis(5));
        }
        // the output language follows the prompt, like the real model
        let es = prompt.prefix.contains("Spanish");
        let raw = if es { "Esta es la línea que dijo." } else { "This is the line that was said." };
        Ok(TranslationOutput { raw: raw.into(), ..Default::default() })
    }
    fn shutdown(&mut self) {}
    fn describe(&self) -> String {
        "fake-mt".into()
    }
}

type Log = Arc<Mutex<Vec<(Instant, PipelineEvent)>>>;

fn run(
    script: &[(f32, f32)],
    asr_delay: Duration,
    mt_delay: Duration,
    die_on_call: Option<u32>,
    cfg: PipelineConfig,
) -> (PipelineHandle, Arc<LanguageControl>, Log) {
    let clock = PipelineClock::new();
    let (cap, _) = ScriptCapture::new(script, clock);
    let lang = LanguageControl::new(TargetLanguage::En);
    let log: Log = Arc::new(Mutex::new(Vec::new()));
    let l2 = log.clone();
    let sink: EventSink = Arc::new(move |e| {
        if !matches!(e, PipelineEvent::Level { .. }) {
            l2.lock().unwrap().push((Instant::now(), e));
        }
    });
    let engines = Engines {
        vad: Box::new(LevelVad),
        asr: Box::new(FakeAsr { delay: asr_delay, n: 0 }),
        translator: Box::new(FakeMt { delay: mt_delay, lang: None, die_on_call, calls: Arc::new(AtomicU32::new(0)) }),
    };
    let h = PipelineHandle::start(cfg, clock, Box::new(cap), engines, lang.clone(), sink);
    (h, lang, log)
}

fn translations(log: &Log) -> Vec<(Instant, u64, TargetLanguage, u64, String)> {
    log.lock()
        .unwrap()
        .iter()
        .filter_map(|(t, e)| match e {
            PipelineEvent::Translation { seg_id, lang, epoch, text, .. } => Some((*t, *seg_id, *lang, *epoch, text.clone())),
            _ => None,
        })
        .collect()
}

fn wait_for(log: &Log, timeout: Duration, f: impl Fn(&[(Instant, PipelineEvent)]) -> bool) -> bool {
    let t0 = Instant::now();
    while t0.elapsed() < timeout {
        if f(&log.lock().unwrap()) {
            return true;
        }
        std::thread::sleep(Duration::from_millis(10));
    }
    false
}

#[test]
fn switch_during_translation_never_publishes_old_language() {
    let script = [(0.8, 0.7), (0.8, 0.7), (0.8, 0.7), (0.8, 1.0)];
    let (h, lang, log) = run(&script, Duration::from_millis(50), Duration::from_millis(400), None, PipelineConfig::default());
    // switch while the first line is being translated
    assert!(wait_for(&log, Duration::from_secs(5), |l| l.iter().any(|(_, e)| matches!(e, PipelineEvent::Transcript { .. }))));
    std::thread::sleep(Duration::from_millis(100));
    let switched_at = Instant::now();
    lang.set(TargetLanguage::Es);
    assert!(wait_for(&log, Duration::from_secs(10), |_| h.is_drained()));
    h.stop();
    let tr = translations(&log);
    assert!(!tr.is_empty(), "no translations at all");
    for (t, seg, l, _, text) in &tr {
        if *t > switched_at {
            assert_eq!(*l, TargetLanguage::Es, "seg {seg} published in {l:?} after switch: {text}");
            assert!(text.starts_with("Esta"), "mixed-language text after switch: {text}");
        }
    }
    // the line in flight at the switch was redone in Spanish, not dropped silently
    assert!(tr.iter().any(|(_, seg, l, ..)| *seg == 1 && *l == TargetLanguage::Es), "{tr:?}");
}

#[test]
fn publication_is_ordered_and_never_duplicated() {
    let script = [(0.6, 0.6); 6];
    let (h, _lang, log) = run(&script, Duration::from_millis(30), Duration::from_millis(30), None, PipelineConfig::default());
    assert!(wait_for(&log, Duration::from_secs(15), |_| h.is_drained()));
    h.stop();
    let ids: Vec<u64> = translations(&log).iter().map(|t| t.1).collect();
    assert_eq!(ids.len(), 6, "{ids:?}");
    assert!(ids.windows(2).all(|w| w[1] > w[0]), "not strictly increasing: {ids:?}");
}

#[test]
fn overload_drops_stale_work_stays_bounded_and_returns_to_live() {
    // ASR far slower than speech arrives: 12 short lines ~1 s apart, 2 s per transcription.
    let script = [(0.5, 0.55); 12];
    let asr_ms = 2_000u64;
    let cfg = PipelineConfig { max_asr_age_ms: 3_000, ..PipelineConfig::default() };
    let (h, _lang, log) = run(&script, Duration::from_millis(asr_ms), Duration::from_millis(10), None, cfg.clone());
    assert!(wait_for(&log, Duration::from_secs(40), |_| h.is_drained()));
    let stats = h.stats();
    h.stop();
    let l = log.lock().unwrap();
    let dropped = l.iter().filter(|(_, e)| matches!(e, PipelineEvent::Dropped { .. })).count();
    assert!(dropped > 0, "nothing was dropped under overload");
    assert!(stats.asr_queue_high_water <= cfg.asr_queue, "queue grew past its bound: {}", stats.asr_queue_high_water);
    assert!(l.iter().any(|(_, e)| matches!(e, PipelineEvent::State { state: PipelineState::FallingBehind, .. })));
    // Delay never accumulates: every transcript appears within max age + one ASR run.
    let bound_ns = (cfg.max_asr_age_ms + asr_ms + 500) * 1_000_000;
    let mut last_seg = 0;
    for (_, e) in l.iter() {
        if let PipelineEvent::Transcript { seg_id, timing, .. } = e {
            let delay = timing.transcript_published_ns - timing.speech_end_ns;
            assert!(delay <= bound_ns, "seg {seg_id} shown {} ms after speech", delay / 1_000_000);
            last_seg = last_seg.max(*seg_id);
        }
    }
    // ...and the pipeline reaches the newest speech instead of lagging behind it.
    assert!(last_seg >= 11, "stuck on old speech: last transcribed seg {last_seg}");
}

#[test]
fn stop_is_prompt_even_mid_inference() {
    let script = [(1.0, 0.6), (1.0, 5.0)];
    let (h, _lang, log) = run(&script, Duration::from_secs(20), Duration::from_millis(10), None, PipelineConfig::default());
    // wait until the ASR stand-in is busy with the first segment
    assert!(wait_for(&log, Duration::from_secs(5), |l| l.iter().any(|(_, e)| matches!(e, PipelineEvent::State { state: PipelineState::Processing, .. }))));
    let report = h.stop();
    assert!(report.threads_joined);
    assert!(report.stop_ms < 500.0, "stop took {} ms", report.stop_ms);
}

#[test]
fn translator_crash_is_reported_and_recovered() {
    let script = [(0.6, 0.7), (0.6, 0.7), (0.6, 1.0)];
    let (h, _lang, log) = run(&script, Duration::from_millis(20), Duration::from_millis(20), Some(1), PipelineConfig::default());
    assert!(wait_for(&log, Duration::from_secs(15), |_| h.is_drained()));
    let stats = h.stats();
    h.stop();
    let l = log.lock().unwrap();
    assert!(l.iter().any(|(_, e)| matches!(e, PipelineEvent::Error { stage, recoverable: true, .. } if stage == "translation")));
    assert!(stats.translator_restarts >= 1);
    drop(l);
    assert_eq!(translations(&log).len(), 3, "lines after the crash were not translated");
}
