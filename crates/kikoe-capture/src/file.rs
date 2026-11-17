//! Decoded-PCM "capture" for the evaluation harness. In real-time mode packets
//! are released on the wall clock exactly as a live device would deliver them,
//! so latency measurements are comparable with live capture.

use std::time::{Duration, Instant};

use kikoe_core::audio::{AudioFormat, AudioPacket, CaptureError, CaptureStats, CaptureStream, SourceInfo, SourceKind};
use kikoe_core::clock::PipelineClock;

pub struct FileCapture {
    info: SourceInfo,
    format: AudioFormat,
    samples: Vec<f32>,
    pos: usize,
    chunk_frames: usize,
    realtime: bool,
    started: Option<Instant>,
    clock: PipelineClock,
    seq: u64,
    stats: CaptureStats,
    stopped: bool,
}

impl FileCapture {
    /// `samples` are interleaved f32. `chunk_ms` mimics the device period (10 ms on WASAPI).
    pub fn new(label: &str, samples: Vec<f32>, format: AudioFormat, clock: PipelineClock, realtime: bool, chunk_ms: u32) -> Self {
        let chunk_frames = (format.sample_rate as usize * chunk_ms as usize / 1000).max(1);
        Self {
            info: SourceInfo { id: label.into(), label: label.into(), kind: SourceKind::File, is_default: false },
            format,
            samples,
            pos: 0,
            chunk_frames,
            realtime,
            started: None,
            clock,
            seq: 0,
            stats: CaptureStats::default(),
            stopped: false,
        }
    }

    /// Seconds of audio already released.
    pub fn position_secs(&self) -> f64 {
        (self.pos / self.format.channels.max(1) as usize) as f64 / self.format.sample_rate as f64
    }
}

impl CaptureStream for FileCapture {
    fn source(&self) -> &SourceInfo {
        &self.info
    }

    fn next_packet(&mut self, timeout: Duration) -> Result<Option<AudioPacket>, CaptureError> {
        if self.stopped || self.pos >= self.samples.len() {
            return Ok(None);
        }
        let ch = self.format.channels.max(1) as usize;
        let start = *self.started.get_or_insert_with(Instant::now);
        let end = (self.pos + self.chunk_frames * ch).min(self.samples.len());
        if self.realtime {
            let due = start + Duration::from_secs_f64((end / ch) as f64 / self.format.sample_rate as f64);
            let now = Instant::now();
            if due > now {
                let wait = due - now;
                if wait > timeout {
                    std::thread::sleep(timeout);
                    return Ok(None);
                }
                std::thread::sleep(wait);
            }
        }
        let samples = self.samples[self.pos..end].to_vec();
        self.pos = end;
        self.seq += 1;
        self.stats.packets += 1;
        self.stats.frames += (samples.len() / ch) as u64;
        Ok(Some(AudioPacket { seq: self.seq, format: self.format, samples, captured_at_ns: self.clock.now_ns(), discontinuity: false }))
    }

    fn is_finished(&self) -> bool {
        self.stopped || self.pos >= self.samples.len()
    }

    fn stats(&self) -> CaptureStats {
        self.stats
    }

    fn stop(&mut self) {
        self.stopped = true;
    }
}
