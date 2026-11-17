//! The live pipeline: three worker threads joined by bounded drop-oldest queues.
//!
//! ```text
//! [audio thread]  capture -> downmix -> resample 16k -> VAD -> segmenter
//!        | DropOldestQueue (asr_queue)
//! [asr thread]    age check -> Whisper -> filters -> transcript published -> context
//!        | DropOldestQueue (mt_queue)
//! [mt thread]     age check -> (lang, epoch) -> prompt -> worker -> sanitize -> coordinator
//! [monitor]       derives Listening / Processing / FallingBehind, emits stats
//! ```
//!
//! Invariants: memory is bounded (queues, series, maps), old audio is dropped
//! rather than translated late, every drop is reported as an event, and `stop`
//! returns only after every thread has exited and the capture is released.

use std::collections::VecDeque;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use std::time::{Duration, Instant};

use serde::{Deserialize, Serialize};

use crate::audio::{ASR_SAMPLE_RATE, CaptureStats, CaptureStream, downmix_to_mono, rms};
use crate::clock::PipelineClock;
use crate::coordinator::{CoordinatorStats, RejectReason, SubtitleCoordinator};
use crate::filters::{AsrEvidence, FilterVerdict, TranscriptFilter};
use crate::lang::{LanguageControl, TargetLanguage};
use crate::metrics::{LatencyBook, LatencyReport, SegmentTiming};
use crate::queue::{DropOldestQueue, Pop};
use crate::resample::Resampler;
use crate::segmenter::{Segmenter, SegmenterConfig, SegmenterStats, SpeechSegment, Vad};
use crate::translate::{
    ContextWindow, PromptStyle, RenderedPrompt, Route, check_plausible, render_assisted_prompt, render_prompt, sanitize_output,
    stop_strings,
};

// ---------------------------------------------------------------------------
// Engine traits (implemented by kikoe-asr and the translation worker client)
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct AsrOutput {
    pub text: String,
    pub no_speech_prob: f32,
    pub avg_token_prob: f32,
    pub tokens: u32,
}

pub trait AsrEngine: Send {
    /// Transcribe 16 kHz mono Japanese audio. Must return promptly once `abort` is set.
    fn transcribe(&mut self, samples: &[f32], abort: &Arc<AtomicBool>) -> Result<AsrOutput, String>;
    fn describe(&self) -> String;
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct TranslationOutput {
    pub raw: String,
    pub prompt_tokens: u32,
    pub cached_tokens: u32,
    pub generated_tokens: u32,
    pub prompt_ms: f64,
    pub generate_ms: f64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TranslateError {
    Cancelled,
    Failed(String),
    /// The worker process died or hung; the engine must be prepared again.
    WorkerDied(String),
}

pub struct CancelFlags<'a> {
    pub stop: &'a AtomicBool,
    pub switch: &'a AtomicBool,
}

impl CancelFlags<'_> {
    pub fn is_set(&self) -> bool {
        self.stop.load(Ordering::Acquire) || self.switch.load(Ordering::Acquire)
    }
}

pub trait TranslationEngine: Send {
    /// Ensure the model serving `lang` is loaded. `Ok(Some(ms))` when a load happened.
    fn prepare(&mut self, lang: TargetLanguage) -> Result<Option<u64>, String>;
    fn prompt_style(&self, lang: TargetLanguage) -> PromptStyle;
    fn translate(
        &mut self,
        prompt: &RenderedPrompt,
        stop: &[String],
        cancel: &CancelFlags<'_>,
    ) -> Result<TranslationOutput, TranslateError>;
    fn shutdown(&mut self);
    fn describe(&self) -> String;
}

// ---------------------------------------------------------------------------
// Configuration, events
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PipelineConfig {
    pub segmenter: SegmenterConfig,
    pub asr_queue: usize,
    pub mt_queue: usize,
    /// A segment whose voiced audio ended longer ago than this is not transcribed.
    pub max_asr_age_ms: u64,
    /// A transcript older than this is shown in Japanese only, not translated.
    pub max_mt_age_ms: u64,
    pub context_lines: usize,
    pub context_gap_ms: u64,
    pub stats_interval_ms: u64,
    /// Live mode drops stale work. The offline evaluator can disable it to
    /// measure engine quality on every segment.
    pub drop_stale: bool,
    /// How Spanish subtitles are produced (English always goes direct).
    pub spanish_route: Route,
}

impl Default for PipelineConfig {
    fn default() -> Self {
        Self {
            segmenter: SegmenterConfig::default(),
            asr_queue: 2,
            mt_queue: 3,
            max_asr_age_ms: 8_000,
            max_mt_age_ms: 12_000,
            context_lines: 2,
            context_gap_ms: 15_000,
            stats_interval_ms: 1_000,
            drop_stale: true,
            spanish_route: crate::models::DEFAULT_SPANISH_ROUTE,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PipelineState {
    Starting,
    Listening,
    Processing,
    FallingBehind,
    Recovering,
    Stopped,
    Failed,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum ModelStatus {
    Loading,
    Ready { load_ms: Option<u64> },
    Failed { message: String },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MtInfo {
    pub prompt_tokens: u32,
    pub cached_tokens: u32,
    pub generated_tokens: u32,
    pub prompt_ms: f64,
    pub generate_ms: f64,
    pub context_lines: usize,
    pub attempts: u32,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct PipelineStats {
    pub capture: CaptureStats,
    pub segmenter: SegmenterStats,
    pub coordinator: CoordinatorStats,
    pub asr_queue_len: usize,
    pub asr_queue_high_water: usize,
    pub mt_queue_len: usize,
    pub mt_queue_high_water: usize,
    pub dropped_asr_queue: u64,
    pub dropped_asr_stale: u64,
    pub dropped_mt_queue: u64,
    pub skipped_mt_stale: u64,
    pub filtered: u64,
    pub translation_failures: u64,
    pub translator_restarts: u64,
    pub language_switches: u64,
    pub latency: LatencyReport,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum PipelineEvent {
    State { state: PipelineState, detail: Option<String> },
    Level { speech_prob: f32, rms: f32, in_speech: bool },
    Transcript { seg_id: u64, ja: String, start_ms: u64, end_ms: u64, timing: SegmentTiming },
    Translation { seg_id: u64, lang: TargetLanguage, epoch: u64, ja: String, text: String, timing: SegmentTiming, mt: MtInfo },
    /// The Japanese line was shown but will not get a translation.
    TranslationSkipped { seg_id: u64, lang: TargetLanguage, reason: String },
    Dropped { seg_id: u64, stage: String, reason: String },
    ModelStatus { lang: TargetLanguage, model: String, status: ModelStatus },
    Stats { stats: Box<PipelineStats> },
    Error { stage: String, message: String, recoverable: bool },
}

pub type EventSink = Arc<dyn Fn(PipelineEvent) + Send + Sync>;

// ---------------------------------------------------------------------------
// Shared state
// ---------------------------------------------------------------------------

#[derive(Default)]
struct Counters {
    dropped_asr_queue: u64,
    dropped_asr_stale: u64,
    dropped_mt_queue: u64,
    skipped_mt_stale: u64,
    filtered: u64,
    translation_failures: u64,
    translator_restarts: u64,
    language_switches: u64,
    capture: CaptureStats,
    segmenter: SegmenterStats,
}

struct Shared {
    clock: PipelineClock,
    stop: Arc<AtomicBool>,
    lang: Arc<LanguageControl>,
    sink: EventSink,
    cfg: PipelineConfig,
    asr_q: DropOldestQueue<AsrJob>,
    mt_q: DropOldestQueue<MtJob>,
    coordinator: Mutex<SubtitleCoordinator>,
    context: Mutex<ContextWindow>,
    latency: Mutex<LatencyBook>,
    counters: Mutex<Counters>,
    published: Mutex<VecDeque<(u64, TargetLanguage, u64)>>,
    last_drop_ns: AtomicU64,
    asr_busy: AtomicBool,
    mt_busy: AtomicBool,
    recovering: AtomicBool,
    failed: AtomicBool,
    finished: AtomicBool,
}

impl Shared {
    fn emit(&self, e: PipelineEvent) {
        (self.sink)(e);
    }

    fn mark_drop(&self) {
        self.last_drop_ns.store(self.clock.now_ns().max(1), Ordering::Release);
    }

    fn stopping(&self) -> bool {
        self.stop.load(Ordering::Acquire)
    }
}

struct AsrJob {
    seg: SpeechSegment,
    timing: SegmentTiming,
}

struct MtJob {
    seg_id: u64,
    ja: String,
    start_ms: u64,
    timing: SegmentTiming,
}

/// Maps 16 kHz stream sample indices back to pipeline-clock time.
struct WallMap {
    marks: VecDeque<(u64, u64)>, // (end sample index, captured_at_ns)
}

impl WallMap {
    fn new() -> Self {
        Self { marks: VecDeque::new() }
    }

    fn mark(&mut self, end_sample: u64, captured_at_ns: u64) {
        self.marks.push_back((end_sample, captured_at_ns));
        while self.marks.len() > 4_000 {
            self.marks.pop_front();
        }
    }

    fn wall_ns(&self, sample: u64) -> u64 {
        let per = 1_000_000_000u64 / ASR_SAMPLE_RATE as u64;
        let mark = self.marks.iter().find(|(end, _)| *end >= sample).or(self.marks.back());
        match mark {
            Some((end, t)) if *end >= sample => t.saturating_sub((end - sample) * per),
            Some((end, t)) => t + (sample - end) * per,
            None => 0,
        }
    }
}

// ---------------------------------------------------------------------------
// Handle
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StopReport {
    pub stop_ms: f64,
    pub threads_joined: bool,
    pub discarded_segments: usize,
    pub discarded_translations: usize,
}

pub struct PipelineHandle {
    shared: Arc<Shared>,
    threads: Vec<JoinHandle<()>>,
}

pub struct Engines {
    pub vad: Box<dyn Vad>,
    pub asr: Box<dyn AsrEngine>,
    pub translator: Box<dyn TranslationEngine>,
}

impl PipelineHandle {
    /// `clock` must be the same clock the capture adapter stamps packets with.
    pub fn start(
        cfg: PipelineConfig,
        clock: PipelineClock,
        capture: Box<dyn CaptureStream>,
        engines: Engines,
        lang: Arc<LanguageControl>,
        sink: EventSink,
    ) -> PipelineHandle {
        let shared = Arc::new(Shared {
            clock,
            stop: Arc::new(AtomicBool::new(false)),
            lang,
            sink,
            asr_q: DropOldestQueue::new(cfg.asr_queue.max(1)),
            mt_q: DropOldestQueue::new(cfg.mt_queue.max(1)),
            coordinator: Mutex::new(SubtitleCoordinator::new()),
            context: Mutex::new(ContextWindow::new(cfg.context_lines, cfg.context_gap_ms)),
            latency: Mutex::new(LatencyBook::default()),
            counters: Mutex::new(Counters::default()),
            published: Mutex::new(VecDeque::new()),
            last_drop_ns: AtomicU64::new(0),
            asr_busy: AtomicBool::new(false),
            mt_busy: AtomicBool::new(false),
            recovering: AtomicBool::new(false),
            failed: AtomicBool::new(false),
            finished: AtomicBool::new(false),
            cfg,
        });
        shared.emit(PipelineEvent::State { state: PipelineState::Starting, detail: None });

        let Engines { vad, asr, translator } = engines;
        let mut threads = Vec::new();
        let s = shared.clone();
        threads.push(spawn("kikoe-audio", move || audio_thread(s, capture, vad)));
        let s = shared.clone();
        threads.push(spawn("kikoe-asr", move || asr_thread(s, asr)));
        let s = shared.clone();
        threads.push(spawn("kikoe-mt", move || mt_thread(s, translator)));
        let s = shared.clone();
        threads.push(spawn("kikoe-monitor", move || monitor_thread(s)));
        PipelineHandle { shared, threads }
    }

    pub fn clock(&self) -> PipelineClock {
        self.shared.clock
    }

    /// True once input ended and every queued item was processed (file sources).
    pub fn is_drained(&self) -> bool {
        self.shared.finished.load(Ordering::Acquire)
    }

    /// The UI reports when it actually rendered a subtitle (for UI latency).
    pub fn record_ui_ack(&self, seg_id: u64, lang: TargetLanguage) {
        let now = self.shared.clock.now_ns();
        let published = self.shared.published.lock().unwrap();
        if let Some((_, l, t)) = published.iter().find(|(id, l, _)| *id == seg_id && *l == lang) {
            let ms = now.saturating_sub(*t) as f64 / 1e6;
            self.shared.latency.lock().unwrap().record_ui(*l, ms);
        }
    }

    pub fn stats(&self) -> PipelineStats {
        collect_stats(&self.shared)
    }

    /// Stop everything and release the capture. Pending work is discarded.
    pub fn stop(self) -> StopReport {
        let t0 = Instant::now();
        self.shared.stop.store(true, Ordering::Release);
        let discarded_segments = self.shared.asr_q.close().len();
        let discarded_translations = self.shared.mt_q.close().len();
        let mut joined = true;
        for t in self.threads {
            joined &= t.join().is_ok();
        }
        let stop_ms = t0.elapsed().as_secs_f64() * 1000.0;
        self.shared.emit(PipelineEvent::State { state: PipelineState::Stopped, detail: Some(format!("stopped in {stop_ms:.0} ms")) });
        StopReport { stop_ms, threads_joined: joined, discarded_segments, discarded_translations }
    }

    /// For file sources: wait until all input has been processed, then stop.
    pub fn wait_drained(self, timeout: Duration) -> StopReport {
        let t0 = Instant::now();
        while !self.is_drained() && t0.elapsed() < timeout {
            std::thread::sleep(Duration::from_millis(20));
        }
        self.stop()
    }
}

fn spawn(name: &str, f: impl FnOnce() + Send + 'static) -> JoinHandle<()> {
    std::thread::Builder::new().name(name.into()).spawn(f).expect("spawn pipeline thread")
}

fn collect_stats(s: &Shared) -> PipelineStats {
    let c = s.counters.lock().unwrap();
    PipelineStats {
        capture: c.capture,
        segmenter: c.segmenter,
        coordinator: s.coordinator.lock().unwrap().stats(),
        asr_queue_len: s.asr_q.len(),
        asr_queue_high_water: s.asr_q.high_water(),
        mt_queue_len: s.mt_q.len(),
        mt_queue_high_water: s.mt_q.high_water(),
        dropped_asr_queue: c.dropped_asr_queue,
        dropped_asr_stale: c.dropped_asr_stale,
        dropped_mt_queue: c.dropped_mt_queue,
        skipped_mt_stale: c.skipped_mt_stale,
        filtered: c.filtered,
        translation_failures: c.translation_failures,
        translator_restarts: c.translator_restarts,
        language_switches: c.language_switches,
        latency: s.latency.lock().unwrap().report(),
    }
}

// ---------------------------------------------------------------------------
// Threads
// ---------------------------------------------------------------------------

fn audio_thread(s: Arc<Shared>, mut capture: Box<dyn CaptureStream>, mut vad: Box<dyn Vad>) {
    let mut segmenter = Segmenter::new(s.cfg.segmenter.clone());
    let mut resampler: Option<Resampler> = None;
    let mut mono = Vec::new();
    let mut pcm16 = Vec::new();
    let mut segs = Vec::new();
    let mut wall = WallMap::new();
    let mut last_level = Instant::now();
    let mut level_rms = 0f32;
    let mut ended = false;

    while !s.stopping() {
        match capture.next_packet(Duration::from_millis(50)) {
            Ok(Some(pkt)) => {
                mono.clear();
                downmix_to_mono(&pkt.samples, pkt.format.channels, &mut mono);
                if resampler.as_ref().is_none_or(|r| r.in_rate() != pkt.format.sample_rate) {
                    resampler = Some(Resampler::new(pkt.format.sample_rate, ASR_SAMPLE_RATE));
                }
                pcm16.clear();
                resampler.as_mut().unwrap().process(&mono, &mut pcm16);
                level_rms = level_rms.max(rms(&pcm16));
                segmenter.push(&pcm16, vad.as_mut(), &mut segs);
                wall.mark(segmenter.stream_position(), pkt.captured_at_ns);
                dispatch_segments(&s, &mut segs, &wall);
            }
            Ok(None) => {
                if capture.is_finished() {
                    ended = true;
                    break;
                }
            }
            Err(e) => {
                s.failed.store(true, Ordering::Release);
                s.emit(PipelineEvent::Error { stage: "capture".into(), message: e.to_string(), recoverable: false });
                s.emit(PipelineEvent::State { state: PipelineState::Failed, detail: Some(e.to_string()) });
                break;
            }
        }
        if last_level.elapsed() >= Duration::from_millis(100) {
            s.emit(PipelineEvent::Level { speech_prob: segmenter.last_probability(), rms: level_rms, in_speech: segmenter.is_in_speech() });
            level_rms = 0.0;
            last_level = Instant::now();
            let mut c = s.counters.lock().unwrap();
            c.capture = capture.stats();
            c.segmenter = segmenter.stats();
        }
    }
    if ended && !s.stopping() {
        if let Some(r) = resampler.as_mut() {
            pcm16.clear();
            r.finish(&mut pcm16);
            segmenter.push(&pcm16, vad.as_mut(), &mut segs);
            wall.mark(segmenter.stream_position(), s.clock.now_ns());
        }
        segmenter.flush(&mut segs);
        dispatch_segments(&s, &mut segs, &wall);
        s.asr_q.finish();
    }
    capture.stop();
    let mut c = s.counters.lock().unwrap();
    c.capture = capture.stats();
    c.segmenter = segmenter.stats();
}

fn dispatch_segments(s: &Shared, segs: &mut Vec<SpeechSegment>, wall: &WallMap) {
    for seg in segs.drain(..) {
        let now = s.clock.now_ns();
        let timing = SegmentTiming {
            seg_id: seg.id,
            speech_start_ns: wall.wall_ns(seg.speech_start_sample),
            speech_end_ns: wall.wall_ns(seg.speech_end_sample).min(now),
            segment_ready_ns: now,
            ..Default::default()
        };
        if let Some(old) = s.asr_q.push(AsrJob { seg, timing }) {
            s.counters.lock().unwrap().dropped_asr_queue += 1;
            s.mark_drop();
            s.emit(PipelineEvent::Dropped { seg_id: old.seg.id, stage: "asr_queue".into(), reason: "falling behind: newer speech arrived".into() });
        }
    }
}

fn sample_ms(sample: u64) -> u64 {
    sample * 1000 / ASR_SAMPLE_RATE as u64
}

fn asr_thread(s: Arc<Shared>, mut asr: Box<dyn AsrEngine>) {
    let mut filter = TranscriptFilter::new();
    loop {
        let job = match s.asr_q.pop_timeout(Duration::from_millis(100)) {
            Pop::Item(j) => j,
            Pop::Timeout => {
                if s.stopping() {
                    break;
                }
                continue;
            }
            Pop::Closed => break,
        };
        if s.stopping() {
            break;
        }
        let AsrJob { seg, mut timing } = job;
        let now = s.clock.now_ns();
        let age_ms = now.saturating_sub(timing.speech_end_ns) / 1_000_000;
        if s.cfg.drop_stale && age_ms > s.cfg.max_asr_age_ms {
            s.counters.lock().unwrap().dropped_asr_stale += 1;
            s.mark_drop();
            s.emit(PipelineEvent::Dropped { seg_id: seg.id, stage: "asr".into(), reason: format!("stale: speech ended {age_ms} ms ago") });
            continue;
        }
        s.asr_busy.store(true, Ordering::Release);
        timing.asr_start_ns = s.clock.now_ns();
        let result = asr.transcribe(&seg.samples, &s.stop);
        timing.asr_end_ns = s.clock.now_ns();
        s.asr_busy.store(false, Ordering::Release);
        if s.stopping() {
            break;
        }
        let out = match result {
            Ok(o) => o,
            Err(e) => {
                s.emit(PipelineEvent::Error { stage: "asr".into(), message: e, recoverable: true });
                continue;
            }
        };
        let ev = AsrEvidence {
            no_speech_prob: out.no_speech_prob,
            avg_token_prob: out.avg_token_prob,
            vad_mean_prob: seg.mean_prob,
            voiced_ms: seg.voiced_ms,
        };
        let start_ms = sample_ms(seg.speech_start_sample);
        let end_ms = sample_ms(seg.speech_end_sample);
        let ja = match filter.check(&out.text, &ev, start_ms, end_ms) {
            FilterVerdict::Keep => crate::filters::normalize_ja(&out.text),
            FilterVerdict::Cleaned(t) => t,
            FilterVerdict::Drop(reason) => {
                s.counters.lock().unwrap().filtered += 1;
                s.emit(PipelineEvent::Dropped {
                    seg_id: seg.id,
                    stage: "filter".into(),
                    reason: format!("{reason:?}: {}", out.text.trim()),
                });
                continue;
            }
        };
        if s.coordinator.lock().unwrap().offer_transcript(seg.id).is_err() {
            continue;
        }
        timing.transcript_published_ns = s.clock.now_ns();
        s.latency.lock().unwrap().record_transcript(&timing);
        s.emit(PipelineEvent::Transcript { seg_id: seg.id, ja: ja.clone(), start_ms, end_ms, timing: timing.clone() });
        s.context.lock().unwrap().push(end_ms, ja.clone());
        if let Some(old) = s.mt_q.push(MtJob { seg_id: seg.id, ja, start_ms, timing }) {
            s.counters.lock().unwrap().dropped_mt_queue += 1;
            s.mark_drop();
            let (lang, _) = s.lang.current();
            s.emit(PipelineEvent::TranslationSkipped { seg_id: old.seg_id, lang, reason: "falling behind: newer line arrived".into() });
        }
    }
    s.asr_busy.store(false, Ordering::Release);
    if !s.stopping() {
        s.mt_q.finish();
    }
}

fn model_name(t: &dyn TranslationEngine) -> String {
    t.describe()
}

fn prepare_for(s: &Shared, t: &mut dyn TranslationEngine, lang: TargetLanguage) -> bool {
    s.emit(PipelineEvent::ModelStatus { lang, model: model_name(t), status: ModelStatus::Loading });
    match t.prepare(lang) {
        Ok(load_ms) => {
            s.recovering.store(false, Ordering::Release);
            s.emit(PipelineEvent::ModelStatus { lang, model: model_name(t), status: ModelStatus::Ready { load_ms } });
            true
        }
        Err(e) => {
            s.emit(PipelineEvent::ModelStatus { lang, model: model_name(t), status: ModelStatus::Failed { message: e.clone() } });
            s.emit(PipelineEvent::Error { stage: "translation".into(), message: e, recoverable: true });
            false
        }
    }
}

fn mt_thread(s: Arc<Shared>, mut t: Box<dyn TranslationEngine>) {
    let switch = s.lang.switch_cancel_flag();
    let mut prepared: Option<TargetLanguage> = None;
    let mut last_prepare_attempt: Option<Instant> = None;
    let mut ensure = |s: &Shared, t: &mut dyn TranslationEngine, prepared: &mut Option<TargetLanguage>, lang: TargetLanguage| -> bool {
        if *prepared == Some(lang) {
            return true;
        }
        // back off between failed attempts so a broken model does not spin
        if let Some(at) = last_prepare_attempt {
            if prepared.is_none() && at.elapsed() < Duration::from_secs(2) {
                std::thread::sleep(Duration::from_secs(2) - at.elapsed());
            }
        }
        last_prepare_attempt = Some(Instant::now());
        if prepare_for(s, t, lang) {
            *prepared = Some(lang);
            true
        } else {
            *prepared = None;
            false
        }
    };

    let (lang0, _) = s.lang.current();
    ensure(&*s, t.as_mut(), &mut prepared, lang0);

    loop {
        let job = match s.mt_q.pop_timeout(Duration::from_millis(100)) {
            Pop::Item(j) => j,
            Pop::Timeout => {
                if s.stopping() {
                    break;
                }
                // follow language switches eagerly so the next line is not delayed by a load
                let (lang, _) = s.lang.current();
                if prepared.is_some() && prepared != Some(lang) {
                    s.counters.lock().unwrap().language_switches += 1;
                    ensure(&*s, t.as_mut(), &mut prepared, lang);
                }
                continue;
            }
            Pop::Closed => break,
        };
        if s.stopping() {
            break;
        }
        s.mt_busy.store(true, Ordering::Release);
        translate_job(&*s, t.as_mut(), &switch, &mut prepared, &mut ensure, job);
        s.mt_busy.store(false, Ordering::Release);
    }
    s.mt_busy.store(false, Ordering::Release);
    t.shutdown();
    if !s.stopping() {
        s.finished.store(true, Ordering::Release);
    }
}

type Ensure<'a> = dyn FnMut(&Shared, &mut dyn TranslationEngine, &mut Option<TargetLanguage>, TargetLanguage) -> bool + 'a;

fn translate_job(
    s: &Shared,
    t: &mut dyn TranslationEngine,
    switch: &AtomicBool,
    prepared: &mut Option<TargetLanguage>,
    ensure: &mut Ensure<'_>,
    job: MtJob,
) {
    let MtJob { seg_id, ja, start_ms, mut timing } = job;
    let context = s.context.lock().unwrap().context_for(start_ms);
    let mut attempts = 0u32;
    loop {
        attempts += 1;
        // Reset the switch flag *before* reading the selection, so a switch that
        // happens after this point always cancels the job we are about to run.
        switch.store(false, Ordering::Release);
        let (lang, epoch) = s.lang.current();
        if *prepared != Some(lang) {
            if prepared.is_some() {
                s.counters.lock().unwrap().language_switches += 1;
            }
            if !ensure(s, &mut *t, prepared, lang) {
                fail(s, seg_id, lang, "translation model unavailable".into());
                return;
            }
        }
        let now = s.clock.now_ns();
        let age_ms = now.saturating_sub(timing.speech_end_ns) / 1_000_000;
        if s.cfg.drop_stale && age_ms > s.cfg.max_mt_age_ms {
            s.counters.lock().unwrap().skipped_mt_stale += 1;
            s.mark_drop();
            s.emit(PipelineEvent::TranslationSkipped { seg_id, lang, reason: format!("stale: speech ended {age_ms} ms ago") });
            return;
        }
        let style = t.prompt_style(lang);
        timing.mt_start_ns = s.clock.now_ns();
        let cancel = CancelFlags { stop: &s.stop, switch };
        let res = translate_line(t, style, lang, s.cfg.spanish_route, &context, &ja, &cancel);
        timing.mt_end_ns = s.clock.now_ns();
        if s.stopping() {
            return;
        }
        match res {
            Ok(out) => {
                let text = match sanitize_output(&out.raw).and_then(|t| check_plausible(&ja, &t).map(|_| t)) {
                    Ok(t) => t,
                    Err(issue) => {
                        fail(s, seg_id, lang, format!("unusable model output ({issue:?})"));
                        return;
                    }
                };
                let current = s.lang.current();
                let decision = {
                    let mut c = s.coordinator.lock().unwrap();
                    c.offer_translation(seg_id, (lang, epoch), current, &text)
                };
                match decision {
                    Ok(()) => {
                        timing.published_ns = s.clock.now_ns();
                        s.latency.lock().unwrap().record_translation(lang, &timing);
                        {
                            let mut p = s.published.lock().unwrap();
                            p.push_back((seg_id, lang, timing.published_ns));
                            while p.len() > 64 {
                                p.pop_front();
                            }
                        }
                        s.emit(PipelineEvent::Translation {
                            seg_id,
                            lang,
                            epoch,
                            ja,
                            text,
                            timing,
                            mt: MtInfo {
                                prompt_tokens: out.prompt_tokens,
                                cached_tokens: out.cached_tokens,
                                generated_tokens: out.generated_tokens,
                                prompt_ms: out.prompt_ms,
                                generate_ms: out.generate_ms,
                                context_lines: context.len(),
                                attempts,
                            },
                        });
                        return;
                    }
                    Err(RejectReason::StaleEpoch) if attempts < 3 => continue, // switched mid-flight: redo in the new language
                    Err(reason) => {
                        fail(s, seg_id, lang, format!("rejected by coordinator: {reason:?}"));
                        return;
                    }
                }
            }
            Err(TranslateError::Cancelled) if attempts < 3 => continue, // language switch: redo
            Err(TranslateError::Cancelled) => {
                fail(s, seg_id, lang, "cancelled".into());
                return;
            }
            Err(TranslateError::WorkerDied(msg)) => {
                s.recovering.store(true, Ordering::Release);
                s.counters.lock().unwrap().translator_restarts += 1;
                s.emit(PipelineEvent::Error { stage: "translation".into(), message: format!("translation worker stopped: {msg}; restarting"), recoverable: true });
                *prepared = None;
                if attempts < 2 {
                    continue;
                }
                fail(s, seg_id, lang, format!("worker failure: {msg}"));
                return;
            }
            Err(TranslateError::Failed(msg)) => {
                fail(s, seg_id, lang, msg);
                return;
            }
        }
    }
}

/// Translate one line by the configured route. An implausible direct Spanish
/// output (example echo / truncation) is retried once through the
/// English-assisted route; anything still implausible is rejected by the caller.
pub fn translate_line(
    t: &mut dyn TranslationEngine,
    style: PromptStyle,
    lang: TargetLanguage,
    spanish_route: Route,
    context: &[String],
    ja: &str,
    cancel: &CancelFlags<'_>,
) -> Result<TranslationOutput, TranslateError> {
    if lang == TargetLanguage::Es && spanish_route == Route::EnglishAssisted {
        return translate_assisted(t, style, context, ja, cancel);
    }
    let out = t.translate(&render_prompt(style, lang, context, ja), &stop_strings(style), cancel)?;
    let plausible = sanitize_output(&out.raw).is_ok_and(|x| check_plausible(ja, &x).is_ok());
    if !plausible && lang == TargetLanguage::Es {
        let mut retry = translate_assisted(t, style, context, ja, cancel)?;
        retry.prompt_ms += out.prompt_ms;
        retry.generate_ms += out.generate_ms;
        return Ok(retry);
    }
    Ok(out)
}

/// Spanish via an English draft: the model gets the Japanese line (source of
/// truth) plus its own English translation as a hint. Falls back to direct
/// Spanish when the draft is unusable.
pub fn translate_assisted(
    t: &mut dyn TranslationEngine,
    style: PromptStyle,
    context: &[String],
    ja: &str,
    cancel: &CancelFlags<'_>,
) -> Result<TranslationOutput, TranslateError> {
    let stop = stop_strings(style);
    let draft = t.translate(&render_prompt(style, TargetLanguage::En, context, ja), &stop, cancel)?;
    let prompt = match sanitize_output(&draft.raw) {
        Ok(d) => render_assisted_prompt(style, TargetLanguage::Es, context, ja, &d),
        Err(_) => render_prompt(style, TargetLanguage::Es, context, ja),
    };
    let mut out = t.translate(&prompt, &stop, cancel)?;
    out.prompt_tokens += draft.prompt_tokens;
    out.cached_tokens += draft.cached_tokens;
    out.generated_tokens += draft.generated_tokens;
    out.prompt_ms += draft.prompt_ms;
    out.generate_ms += draft.generate_ms;
    Ok(out)
}

fn fail(s: &Shared, seg_id: u64, lang: TargetLanguage, reason: String) {
    s.counters.lock().unwrap().translation_failures += 1;
    s.emit(PipelineEvent::TranslationSkipped { seg_id, lang, reason });
}

fn monitor_thread(s: Arc<Shared>) {
    let mut last_state = PipelineState::Starting;
    let mut last_stats = Instant::now();
    while !s.stopping() && !s.finished.load(Ordering::Acquire) {
        std::thread::sleep(Duration::from_millis(100));
        let now = s.clock.now_ns();
        let last_drop = s.last_drop_ns.load(Ordering::Acquire);
        let behind = (last_drop > 0 && now.saturating_sub(last_drop) < 8_000_000_000)
            || s.asr_q.len() >= s.asr_q.capacity()
            || s.mt_q.len() >= s.mt_q.capacity();
        if s.failed.load(Ordering::Acquire) {
            last_state = PipelineState::Failed;
            continue;
        }
        let state = if s.recovering.load(Ordering::Acquire) {
            PipelineState::Recovering
        } else if behind {
            PipelineState::FallingBehind
        } else if s.asr_busy.load(Ordering::Acquire) || s.mt_busy.load(Ordering::Acquire) || !s.asr_q.is_empty() || !s.mt_q.is_empty() {
            PipelineState::Processing
        } else {
            PipelineState::Listening
        };
        if state != last_state {
            s.emit(PipelineEvent::State { state, detail: None });
            last_state = state;
        }
        if last_stats.elapsed() >= Duration::from_millis(s.cfg.stats_interval_ms) {
            s.emit(PipelineEvent::Stats { stats: Box::new(collect_stats(&s)) });
            last_stats = Instant::now();
        }
    }
    if !s.stopping() {
        s.emit(PipelineEvent::Stats { stats: Box::new(collect_stats(&s)) });
    }
}
