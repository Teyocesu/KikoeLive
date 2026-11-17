//! The audio contract every platform adapter must satisfy.
//!
//! * Samples are interleaved `f32` in [-1.0, 1.0] at the device's native rate and
//!   channel count; adapters convert integer formats before handing packets over.
//! * `seq` increases by exactly one per packet within a capture session; a gap means
//!   the adapter itself dropped packets (reported in [`CaptureStats`]).
//! * `captured_at_ns` is the pipeline-clock time at which the adapter received the
//!   packet's last frame from the OS.
//! * There is intentionally no microphone source kind: KikoeLive never substitutes
//!   microphone capture for system audio.

use std::time::Duration;

use serde::{Deserialize, Serialize};

pub const ASR_SAMPLE_RATE: u32 = 16_000;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SourceKind {
    /// Everything the OS is playing (WASAPI loopback / ScreenCaptureKit audio).
    SystemLoopback,
    /// A pre-recorded file, used only by the evaluation harness.
    File,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceInfo {
    /// Stable identifier (device id on Windows, display id on macOS, path for files).
    pub id: String,
    /// Human readable label for the UI.
    pub label: String,
    pub kind: SourceKind,
    /// True when this source is the OS default output.
    pub is_default: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AudioFormat {
    pub sample_rate: u32,
    pub channels: u16,
}

#[derive(Clone, Debug)]
pub struct AudioPacket {
    pub seq: u64,
    pub format: AudioFormat,
    /// Interleaved f32 samples, `frames * channels` long.
    pub samples: Vec<f32>,
    pub captured_at_ns: u64,
    /// The OS reported a discontinuity (glitch / dropped buffer) before this packet.
    pub discontinuity: bool,
}

impl AudioPacket {
    pub fn frames(&self) -> usize {
        if self.format.channels == 0 {
            0
        } else {
            self.samples.len() / self.format.channels as usize
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize)]
pub struct CaptureStats {
    pub packets: u64,
    pub frames: u64,
    /// Packets the adapter could not hand over because the pipeline was full.
    pub dropped_packets: u64,
    /// Discontinuities reported by the OS.
    pub discontinuities: u64,
    /// Times the adapter re-opened the device (e.g. default output changed).
    pub reopen_count: u64,
}

#[derive(Debug, thiserror::Error)]
pub enum CaptureError {
    #[error("permission required: {0}")]
    PermissionDenied(String),
    #[error("no system audio source available: {0}")]
    NoSource(String),
    #[error("audio device error: {0}")]
    Device(String),
    #[error("unsupported on this OS version: {0}")]
    Unsupported(String),
}

/// A running capture. Implementations own their OS thread / callbacks and must:
/// * deliver packets through `next_packet` in `seq` order,
/// * make `stop` idempotent, and release every OS resource before it returns,
/// * never block the OS audio thread on the consumer (drop + count instead).
pub trait CaptureStream: Send {
    fn source(&self) -> &SourceInfo;
    /// Wait up to `timeout` for the next packet. `Ok(None)` means "nothing yet"
    /// (system audio is silent or paused), not end of stream.
    fn next_packet(&mut self, timeout: Duration) -> Result<Option<AudioPacket>, CaptureError>;
    /// `true` once the stream can never produce more packets (file ended, stopped).
    fn is_finished(&self) -> bool;
    fn stats(&self) -> CaptureStats;
    fn stop(&mut self);
}

/// Average all channels into mono, appending to `out`.
pub fn downmix_to_mono(samples: &[f32], channels: u16, out: &mut Vec<f32>) {
    let ch = channels.max(1) as usize;
    if ch == 1 {
        out.extend_from_slice(samples);
        return;
    }
    let scale = 1.0 / ch as f32;
    out.reserve(samples.len() / ch);
    for frame in samples.chunks_exact(ch) {
        let sum: f32 = frame.iter().sum();
        out.push(sum * scale);
    }
}

pub fn rms(samples: &[f32]) -> f32 {
    if samples.is_empty() {
        return 0.0;
    }
    let sum: f64 = samples.iter().map(|s| (*s as f64) * (*s as f64)).sum();
    (sum / samples.len() as f64).sqrt() as f32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn downmix_averages_stereo() {
        let mut out = Vec::new();
        downmix_to_mono(&[1.0, 0.0, 0.5, 0.5, -1.0, 1.0], 2, &mut out);
        assert_eq!(out, vec![0.5, 0.5, 0.0]);
    }

    #[test]
    fn downmix_drops_incomplete_trailing_frame() {
        let mut out = Vec::new();
        downmix_to_mono(&[1.0, 1.0, 1.0], 2, &mut out);
        assert_eq!(out, vec![1.0]);
    }
}
