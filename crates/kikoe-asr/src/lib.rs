//! whisper.cpp adapters: Japanese ASR and Silero VAD (both via whisper-rs).
//!
//! Decoding settings follow the Phase 1 lessons:
//! * `no_context`: never condition on the previous segment (it caused loops and
//!   repeated subtitles across segment boundaries),
//! * single segment, no timestamps: segments are already phrase-sized,
//! * suppress blank and non-speech tokens, temperature fallback on low confidence,
//! * the abort callback is wired to the pipeline's stop flag so Stop is prompt.

use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use kikoe_core::pipeline::{AsrEngine, AsrOutput};
use kikoe_core::segmenter::{VAD_FRAME, Vad};
use whisper_rs::{
    FullParams, SamplingStrategy, WhisperContext, WhisperContextParameters, WhisperState, WhisperVadContext,
    WhisperVadContextParams,
};

pub fn install_logging() {
    whisper_rs::install_logging_hooks();
}

pub fn gpu_backend_compiled() -> &'static str {
    if cfg!(feature = "cuda") {
        "cuda"
    } else if cfg!(feature = "metal") {
        "metal"
    } else {
        "cpu"
    }
}

#[derive(Clone, Debug)]
pub struct WhisperConfig {
    pub model_path: PathBuf,
    pub use_gpu: bool,
    pub threads: i32,
    /// 1 = greedy; >1 = beam search width.
    pub beam_size: i32,
    pub initial_prompt: Option<String>,
}

pub struct WhisperAsr {
    _ctx: WhisperContext,
    state: WhisperState,
    cfg: WhisperConfig,
    eot: i32,
    describe: String,
}

impl WhisperAsr {
    pub fn load(cfg: WhisperConfig) -> Result<Self, String> {
        let path = cfg.model_path.to_str().ok_or("model path is not valid UTF-8")?;
        let mut params = WhisperContextParameters::default();
        params.use_gpu(cfg.use_gpu).flash_attn(cfg.use_gpu);
        let ctx = WhisperContext::new_with_params(path, params).map_err(|e| format!("loading Whisper model: {e}"))?;
        let state = ctx.create_state().map_err(|e| format!("creating Whisper state: {e}"))?;
        let eot = ctx.token_eot();
        let name = Path::new(path).file_name().map(|f| f.to_string_lossy().into_owned()).unwrap_or_default();
        let describe = format!(
            "{name} ({}, {})",
            if cfg.use_gpu { gpu_backend_compiled() } else { "cpu" },
            if cfg.beam_size > 1 { format!("beam {}", cfg.beam_size) } else { "greedy".into() }
        );
        Ok(Self { _ctx: ctx, state, cfg, eot, describe })
    }
}

impl AsrEngine for WhisperAsr {
    fn transcribe(&mut self, samples: &[f32], abort: &Arc<AtomicBool>) -> Result<AsrOutput, String> {
        let strategy = if self.cfg.beam_size > 1 {
            SamplingStrategy::BeamSearch { beam_size: self.cfg.beam_size, patience: -1.0 }
        } else {
            SamplingStrategy::Greedy { best_of: 1 }
        };
        let mut p = FullParams::new(strategy);
        p.set_language(Some("ja"));
        p.set_translate(false);
        p.set_no_context(true);
        p.set_single_segment(true);
        p.set_no_timestamps(true);
        p.set_suppress_blank(true);
        p.set_suppress_nst(true);
        p.set_temperature(0.0);
        p.set_temperature_inc(0.2);
        p.set_entropy_thold(2.4);
        p.set_logprob_thold(-1.0);
        p.set_no_speech_thold(0.6);
        p.set_print_special(false);
        p.set_print_progress(false);
        p.set_print_realtime(false);
        p.set_print_timestamps(false);
        p.set_n_threads(self.cfg.threads);
        if let Some(prompt) = &self.cfg.initial_prompt {
            p.set_initial_prompt(prompt);
        }
        let a = abort.clone();
        p.set_abort_callback_safe(move || a.load(Ordering::Relaxed));

        // whisper.cpp refuses input shorter than 100 ms; pad very short clips.
        let min_len = 16_000 / 4;
        let padded;
        let audio: &[f32] = if samples.len() < min_len {
            let mut v = samples.to_vec();
            v.resize(min_len, 0.0);
            padded = v;
            &padded
        } else {
            samples
        };
        self.state.full(p, audio).map_err(|e| {
            if abort.load(Ordering::Relaxed) { "aborted".to_string() } else { format!("whisper failed: {e}") }
        })?;

        let mut text = String::new();
        let mut no_speech = 0f32;
        let mut probs_sum = 0f32;
        let mut probs_n = 0u32;
        let n = self.state.full_n_segments();
        for i in 0..n {
            let Some(seg) = self.state.get_segment(i) else { continue };
            if i == 0 {
                no_speech = seg.no_speech_probability();
            }
            if let Ok(t) = seg.to_str_lossy() {
                text.push_str(&t);
            }
            for k in 0..seg.n_tokens() {
                if let Some(tok) = seg.get_token(k) {
                    if tok.token_id() < self.eot {
                        probs_sum += tok.token_probability();
                        probs_n += 1;
                    }
                }
            }
        }
        Ok(AsrOutput {
            text: text.trim().to_string(),
            no_speech_prob: no_speech,
            avg_token_prob: if probs_n > 0 { probs_sum / probs_n as f32 } else { 0.0 },
            tokens: probs_n,
        })
    }

    fn describe(&self) -> String {
        self.describe.clone()
    }
}

/// Silero VAD through whisper.cpp's ggml port.
///
/// whisper.cpp resets the LSTM state on every `detect_speech` call, so each new
/// 32 ms frame is scored together with the preceding `context_frames` frames as
/// warm-up and only the last probability is used.
pub struct SileroVad {
    ctx: WhisperVadContext,
    window: Vec<f32>,
    context_frames: usize,
}

impl SileroVad {
    pub fn load(model_path: &Path, context_frames: usize) -> Result<Self, String> {
        let path = model_path.to_str().ok_or("VAD model path is not valid UTF-8")?;
        let mut params = WhisperVadContextParams::new();
        params.set_n_threads(1);
        params.set_use_gpu(false);
        let ctx = WhisperVadContext::new(path, params).map_err(|e| format!("loading Silero VAD: {e}"))?;
        Ok(Self { ctx, window: Vec::with_capacity((context_frames + 1) * VAD_FRAME), context_frames: context_frames.max(1) })
    }
}

impl Vad for SileroVad {
    fn frame_probability(&mut self, frame: &[f32]) -> f32 {
        self.window.extend_from_slice(frame);
        let max = (self.context_frames + 1) * VAD_FRAME;
        if self.window.len() > max {
            let excess = self.window.len() - max;
            self.window.drain(..excess);
        }
        if self.ctx.detect_speech(&self.window).is_err() {
            return 0.0;
        }
        self.ctx.probabilities().last().copied().unwrap_or(0.0)
    }

    fn reset(&mut self) {
        self.window.clear();
    }

    fn name(&self) -> &str {
        "silero-v6.2"
    }
}
