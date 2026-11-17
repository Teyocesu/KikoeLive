//! Speech segmentation on top of frame-level VAD probabilities.
//!
//! Goals (from the Phase 1 findings):
//! * phrase-sized segments: close on a natural pause, not mid-predicate,
//! * never wait indefinitely: a soft limit shortens the pause needed to close,
//!   a hard limit cuts at the quietest frame of the recent window,
//! * no overlap between consecutive segments, so ASR never sees the same audio
//!   twice (overlap was a source of repeated subtitles),
//! * blips shorter than `min_speech_ms` of voiced audio are discarded (counted).

use std::collections::VecDeque;

use serde::{Deserialize, Serialize};

use crate::audio::{ASR_SAMPLE_RATE, rms};

/// Silero's native window at 16 kHz (32 ms).
pub const VAD_FRAME: usize = 512;
const FRAME_MS: f32 = VAD_FRAME as f32 * 1000.0 / ASR_SAMPLE_RATE as f32;

pub trait Vad: Send {
    /// Speech probability for exactly one `VAD_FRAME`-sample frame of 16 kHz mono audio.
    fn frame_probability(&mut self, frame: &[f32]) -> f32;
    fn reset(&mut self);
    fn name(&self) -> &str;
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SegmenterConfig {
    pub onset: f32,
    pub offset: f32,
    pub min_speech_ms: u32,
    pub pre_roll_ms: u32,
    pub post_roll_ms: u32,
    pub end_silence_ms: u32,
    pub soft_max_ms: u32,
    pub soft_end_silence_ms: u32,
    pub max_segment_ms: u32,
    pub cut_search_ms: u32,
}

impl Default for SegmenterConfig {
    fn default() -> Self {
        Self {
            onset: 0.5,
            offset: 0.35,
            min_speech_ms: 220,
            pre_roll_ms: 192,
            post_roll_ms: 160,
            end_silence_ms: 448,
            soft_max_ms: 4_000,
            soft_end_silence_ms: 192,
            max_segment_ms: 7_000,
            cut_search_ms: 1_600,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CutReason {
    Pause,
    MaxLength,
    Flush,
}

#[derive(Clone, Debug)]
pub struct SpeechSegment {
    pub id: u64,
    /// 16 kHz mono samples including pre/post roll.
    pub samples: Vec<f32>,
    /// Stream sample index of `samples[0]`.
    pub start_sample: u64,
    /// Stream sample index where voiced audio begins (after pre-roll).
    pub speech_start_sample: u64,
    /// Stream sample index where the last voiced frame ends.
    pub speech_end_sample: u64,
    pub cut: CutReason,
    pub voiced_ms: u32,
    pub mean_prob: f32,
}

impl SpeechSegment {
    pub fn duration_ms(&self) -> u32 {
        (self.samples.len() as u64 * 1000 / ASR_SAMPLE_RATE as u64) as u32
    }
}

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize)]
pub struct SegmenterStats {
    pub segments: u64,
    pub discarded_blips: u64,
    pub forced_cuts: u64,
}

struct Frame {
    samples: Vec<f32>,
    prob: f32,
    index: u64,
}

pub struct Segmenter {
    cfg: SegmenterConfig,
    pending: Vec<f32>,
    frame_index: u64,
    history: VecDeque<Frame>,
    active: Vec<Frame>,
    in_speech: bool,
    silence_run: u32,
    next_id: u64,
    stats: SegmenterStats,
    last_prob: f32,
}

fn ms_to_frames(ms: u32) -> usize {
    ((ms as f32) / FRAME_MS).round().max(1.0) as usize
}

impl Segmenter {
    pub fn new(cfg: SegmenterConfig) -> Self {
        Self {
            cfg,
            pending: Vec::with_capacity(VAD_FRAME * 2),
            frame_index: 0,
            history: VecDeque::new(),
            active: Vec::new(),
            in_speech: false,
            silence_run: 0,
            next_id: 1,
            stats: SegmenterStats::default(),
            last_prob: 0.0,
        }
    }

    pub fn stats(&self) -> SegmenterStats {
        self.stats
    }

    /// Most recent frame probability (for a "hearing speech" indicator).
    pub fn last_probability(&self) -> f32 {
        self.last_prob
    }

    pub fn is_in_speech(&self) -> bool {
        self.in_speech
    }

    /// Stream sample index of the next sample that will be pushed.
    pub fn stream_position(&self) -> u64 {
        self.frame_index * VAD_FRAME as u64 + self.pending.len() as u64
    }

    pub fn push(&mut self, samples: &[f32], vad: &mut dyn Vad, out: &mut Vec<SpeechSegment>) {
        self.pending.extend_from_slice(samples);
        let mut offset = 0;
        while self.pending.len() - offset >= VAD_FRAME {
            let frame: Vec<f32> = self.pending[offset..offset + VAD_FRAME].to_vec();
            offset += VAD_FRAME;
            let prob = vad.frame_probability(&frame);
            self.on_frame(Frame { samples: frame, prob, index: self.frame_index }, out);
            self.frame_index += 1;
        }
        if offset > 0 {
            self.pending.drain(..offset);
        }
    }

    /// End of stream / Stop: emit whatever speech is in progress.
    pub fn flush(&mut self, out: &mut Vec<SpeechSegment>) {
        if self.in_speech {
            self.close_segment(CutReason::Flush, out);
        }
        self.history.clear();
    }

    fn on_frame(&mut self, frame: Frame, out: &mut Vec<SpeechSegment>) {
        self.last_prob = frame.prob;
        if !self.in_speech {
            if frame.prob >= self.cfg.onset {
                self.in_speech = true;
                self.silence_run = 0;
                self.active = self.history.drain(..).collect();
                self.active.push(frame);
            } else {
                self.history.push_back(frame);
                let keep = ms_to_frames(self.cfg.pre_roll_ms);
                while self.history.len() > keep {
                    self.history.pop_front();
                }
            }
            return;
        }

        let voiced = frame.prob >= self.cfg.offset;
        self.active.push(frame);
        if voiced {
            self.silence_run = 0;
        } else {
            self.silence_run += 1;
        }

        let len_ms = self.active.len() as f32 * FRAME_MS;
        let needed_silence =
            if len_ms > self.cfg.soft_max_ms as f32 { self.cfg.soft_end_silence_ms } else { self.cfg.end_silence_ms };
        if self.silence_run as usize >= ms_to_frames(needed_silence) {
            self.close_segment(CutReason::Pause, out);
            return;
        }
        if len_ms >= self.cfg.max_segment_ms as f32 {
            self.forced_cut(out);
        }
    }

    /// Hard limit reached: cut at the least speech-like frame in the search window,
    /// carry the remainder into a new segment (no overlap, nothing lost).
    fn forced_cut(&mut self, out: &mut Vec<SpeechSegment>) {
        self.stats.forced_cuts += 1;
        let n = self.active.len();
        let search = ms_to_frames(self.cfg.cut_search_ms).min(n.saturating_sub(1));
        let lo = n - search;
        // Leave at least ~0.5 s in the carried remainder so the next segment is not a stub.
        let hi = n.saturating_sub(ms_to_frames(500)).max(lo + 1).min(n);
        let mut best = lo;
        let mut best_score = f32::MAX;
        for i in lo..hi {
            // prefer low probability, break ties with low energy
            let score = self.active[i].prob + rms(&self.active[i].samples) * 0.5;
            if score < best_score {
                best_score = score;
                best = i;
            }
        }
        let remainder: Vec<Frame> = self.active.drain(best + 1..).collect();
        self.emit(CutReason::MaxLength, out, false);
        // continue in speech with the remainder
        self.active = remainder;
        self.silence_run = self
            .active
            .iter()
            .rev()
            .take_while(|f| f.prob < self.cfg.offset)
            .count() as u32;
        self.in_speech = !self.active.is_empty();
    }

    fn close_segment(&mut self, reason: CutReason, out: &mut Vec<SpeechSegment>) {
        self.emit(reason, out, true);
        self.active.clear();
        self.in_speech = false;
        self.silence_run = 0;
    }

    fn emit(&mut self, reason: CutReason, out: &mut Vec<SpeechSegment>, trim_tail: bool) {
        let offset = self.cfg.offset;
        let Some(first_voiced) = self.active.iter().position(|f| f.prob >= offset) else {
            self.stats.discarded_blips += 1;
            return;
        };
        let last_voiced = self.active.iter().rposition(|f| f.prob >= offset).unwrap_or(first_voiced);
        let end = if trim_tail {
            (last_voiced + 1 + ms_to_frames(self.cfg.post_roll_ms)).min(self.active.len())
        } else {
            self.active.len()
        };
        let frames = &self.active[..end];
        let voiced: Vec<&Frame> = frames.iter().filter(|f| f.prob >= offset).collect();
        let voiced_ms = (voiced.len() as f32 * FRAME_MS) as u32;
        if voiced_ms < self.cfg.min_speech_ms {
            self.stats.discarded_blips += 1;
            return;
        }
        let mean_prob = voiced.iter().map(|f| f.prob).sum::<f32>() / voiced.len() as f32;
        let mut samples = Vec::with_capacity(frames.len() * VAD_FRAME);
        for f in frames {
            samples.extend_from_slice(&f.samples);
        }
        let seg = SpeechSegment {
            id: self.next_id,
            samples,
            start_sample: frames[0].index * VAD_FRAME as u64,
            speech_start_sample: self.active[first_voiced].index * VAD_FRAME as u64,
            speech_end_sample: (self.active[last_voiced].index + 1) * VAD_FRAME as u64,
            cut: reason,
            voiced_ms,
            mean_prob,
        };
        self.next_id += 1;
        self.stats.segments += 1;
        out.push(seg);
    }
}

/// Adaptive-noise-floor energy VAD. This is the Phase 1-style RMS approach, kept
/// as a measured baseline for the evaluation harness; the app uses Silero.
pub struct EnergyVad {
    noise_db: f32,
}

impl EnergyVad {
    pub fn new() -> Self {
        Self { noise_db: -60.0 }
    }
}

impl Default for EnergyVad {
    fn default() -> Self {
        Self::new()
    }
}

impl Vad for EnergyVad {
    fn frame_probability(&mut self, frame: &[f32]) -> f32 {
        let db = 20.0 * rms(frame).max(1e-6).log10();
        // track the floor quickly downward, slowly upward
        if db < self.noise_db {
            self.noise_db = 0.7 * self.noise_db + 0.3 * db;
        } else {
            self.noise_db = 0.995 * self.noise_db + 0.005 * db;
        }
        let snr = db - self.noise_db.max(-70.0);
        1.0 / (1.0 + (-(snr - 12.0) / 3.0).exp())
    }

    fn reset(&mut self) {
        self.noise_db = -60.0;
    }

    fn name(&self) -> &str {
        "energy"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Deterministic VAD for tests: probability is taken from the frame's first sample.
    struct ScriptVad;
    impl Vad for ScriptVad {
        fn frame_probability(&mut self, frame: &[f32]) -> f32 {
            frame[0]
        }
        fn reset(&mut self) {}
        fn name(&self) -> &str {
            "script"
        }
    }

    fn frames(prob: f32, ms: u32) -> Vec<f32> {
        let n = ms_to_frames(ms) * VAD_FRAME;
        let mut v = vec![0.0f32; n];
        for f in v.chunks_mut(VAD_FRAME) {
            f[0] = prob;
        }
        v
    }

    fn run(input: &[f32], cfg: SegmenterConfig) -> (Vec<SpeechSegment>, SegmenterStats) {
        let mut s = Segmenter::new(cfg);
        let mut out = Vec::new();
        let mut vad = ScriptVad;
        for c in input.chunks(160) {
            s.push(c, &mut vad, &mut out);
        }
        s.flush(&mut out);
        (out, s.stats())
    }

    #[test]
    fn closes_on_pause_and_keeps_preroll() {
        let mut a = frames(0.0, 1000);
        a.extend(frames(0.9, 1200));
        a.extend(frames(0.0, 1000));
        let (segs, _) = run(&a, SegmenterConfig::default());
        assert_eq!(segs.len(), 1);
        let s = &segs[0];
        assert_eq!(s.cut, CutReason::Pause);
        assert!(s.speech_start_sample > s.start_sample, "pre-roll present");
        let voiced_start_ms = s.speech_start_sample * 1000 / 16_000;
        assert!((990..=1030).contains(&voiced_start_ms), "{voiced_start_ms}");
    }

    #[test]
    fn short_pause_inside_phrase_does_not_split() {
        // 200 ms pause (e.g. between 行か and ない) is below end_silence_ms
        let mut a = frames(0.9, 900);
        a.extend(frames(0.1, 200));
        a.extend(frames(0.9, 700));
        a.extend(frames(0.0, 800));
        let (segs, _) = run(&a, SegmenterConfig::default());
        assert_eq!(segs.len(), 1, "split inside phrase");
    }

    #[test]
    fn long_speech_is_cut_without_overlap_or_loss() {
        let mut a = frames(0.9, 3000);
        a.extend(frames(0.4, 200)); // weak dip: best place to cut
        a.extend(frames(0.9, 6000));
        a.extend(frames(0.0, 800));
        let cfg = SegmenterConfig { soft_max_ms: 100_000, ..Default::default() };
        let (segs, stats) = run(&a, cfg.clone());
        assert!(stats.forced_cuts >= 1);
        assert!(segs.len() >= 2);
        for w in segs.windows(2) {
            let a_end = w[0].start_sample + w[0].samples.len() as u64;
            assert!(w[1].start_sample >= a_end, "segments overlap");
        }
        for s in &segs {
            assert!(s.duration_ms() <= cfg.max_segment_ms + 40, "too long: {}", s.duration_ms());
        }
    }

    #[test]
    fn blips_are_discarded_and_counted() {
        let mut a = frames(0.0, 500);
        a.extend(frames(0.9, 64));
        a.extend(frames(0.0, 1000));
        let (segs, stats) = run(&a, SegmenterConfig::default());
        assert!(segs.is_empty());
        assert_eq!(stats.discarded_blips, 1);
    }

    #[test]
    fn soft_limit_closes_on_shorter_pause() {
        let mut a = frames(0.9, 4500);
        a.extend(frames(0.0, 260)); // shorter than end_silence, longer than soft_end_silence
        a.extend(frames(0.9, 1000));
        a.extend(frames(0.0, 800));
        let (segs, _) = run(&a, SegmenterConfig::default());
        assert_eq!(segs.len(), 2);
        assert_eq!(segs[0].cut, CutReason::Pause);
    }

    #[test]
    fn ids_are_monotonic() {
        let mut a = Vec::new();
        for _ in 0..5 {
            a.extend(frames(0.9, 600));
            a.extend(frames(0.0, 700));
        }
        let (segs, _) = run(&a, SegmenterConfig::default());
        assert_eq!(segs.len(), 5);
        for w in segs.windows(2) {
            assert!(w[1].id > w[0].id);
        }
    }
}
