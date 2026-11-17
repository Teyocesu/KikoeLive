//! ScreenCaptureKit system-audio capture (macOS 13+).
//!
//! STATUS: written against the `screencapturekit` 11.x API but NOT compiled or
//! run in this implementation round (developed on Windows; see HANDOFF.md).
//!
//! * Captures the display's audio mix with `excludes_current_process_audio`, so
//!   KikoeLive can never hear itself.
//! * Video is configured to the minimum (2x2 px, 1 fps) because ScreenCaptureKit
//!   requires a video configuration even for audio-only use.
//! * Requires the Screen Recording permission; when it is missing,
//!   `SCShareableContent::get` fails and the user is told exactly what to enable.

use std::sync::mpsc::{Receiver, RecvTimeoutError, SyncSender, TrySendError, sync_channel};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use kikoe_core::audio::{AudioFormat, AudioPacket, CaptureError, CaptureStats, CaptureStream, SourceInfo, SourceKind};
use kikoe_core::clock::PipelineClock;
use screencapturekit::prelude::*;

const SAMPLE_RATE: u32 = 48_000;
const CHANNELS: u16 = 2;
const PERMISSION_HELP: &str = "KikoeLive needs the Screen & System Audio Recording permission: System Settings > \
Privacy & Security > Screen & System Audio Recording > enable KikoeLive, then restart KikoeLive.";

fn shareable() -> Result<SCShareableContent, CaptureError> {
    SCShareableContent::get().map_err(|e| CaptureError::PermissionDenied(format!("{PERMISSION_HELP} ({e:?})")))
}

pub fn list_sources() -> Result<Vec<SourceInfo>, CaptureError> {
    let content = shareable()?;
    Ok(content
        .displays()
        .into_iter()
        .enumerate()
        .map(|(i, d)| SourceInfo {
            id: d.display_id().to_string(),
            label: if i == 0 { "System audio (all apps)".into() } else { format!("System audio (display {})", i + 1) },
            kind: SourceKind::SystemLoopback,
            is_default: i == 0,
        })
        .collect())
}

struct AudioHandler {
    tx: SyncSender<AudioPacket>,
    clock: PipelineClock,
    stats: Arc<Mutex<CaptureStats>>,
    seq: Mutex<u64>,
}

impl SCStreamOutputTrait for AudioHandler {
    fn did_output_sample_buffer(&self, sample: CMSampleBuffer, of_type: SCStreamOutputType) {
        if !matches!(of_type, SCStreamOutputType::Audio) {
            return;
        }
        let Ok(list) = sample.audio_buffer_list() else { return };
        let n_buf = list.num_buffers();
        let mut interleaved: Vec<f32> = Vec::new();
        if n_buf >= 2 {
            // planar float32: one buffer per channel
            let planes: Vec<&[f32]> = (0..n_buf.min(CHANNELS as usize))
                .filter_map(|i| list.get(i))
                .map(|b| bytes_as_f32(b.data()))
                .collect();
            let frames = planes.iter().map(|p| p.len()).min().unwrap_or(0);
            interleaved.reserve(frames * planes.len());
            for f in 0..frames {
                for p in &planes {
                    interleaved.push(p[f]);
                }
            }
            if planes.len() == 1 {
                // degenerate: duplicate to the declared channel count
                interleaved = interleaved.iter().flat_map(|s| [*s, *s]).collect();
            }
        } else if let Some(b) = list.get(0) {
            let data = bytes_as_f32(b.data());
            if b.number_channels >= 2 {
                interleaved.extend_from_slice(data);
            } else {
                interleaved = data.iter().flat_map(|s| [*s, *s]).collect();
            }
        }
        if interleaved.is_empty() {
            return;
        }
        let seq = {
            let mut s = self.seq.lock().unwrap();
            *s += 1;
            *s
        };
        let frames = (interleaved.len() / CHANNELS as usize) as u64;
        let pkt = AudioPacket {
            seq,
            format: AudioFormat { sample_rate: SAMPLE_RATE, channels: CHANNELS },
            samples: interleaved,
            captured_at_ns: self.clock.now_ns(),
            discontinuity: false,
        };
        match self.tx.try_send(pkt) {
            Ok(()) => {
                let mut s = self.stats.lock().unwrap();
                s.packets += 1;
                s.frames += frames;
            }
            Err(TrySendError::Full(_)) => self.stats.lock().unwrap().dropped_packets += 1,
            Err(TrySendError::Disconnected(_)) => {}
        }
    }
}

fn bytes_as_f32(b: &[u8]) -> &[f32] {
    // CoreAudio buffers are 16-byte aligned float32 for ScreenCaptureKit audio.
    let n = b.len() / 4;
    if n == 0 || (b.as_ptr() as usize) % std::mem::align_of::<f32>() != 0 {
        return &[];
    }
    unsafe { std::slice::from_raw_parts(b.as_ptr() as *const f32, n) }
}

pub struct ScreenCaptureAudio {
    info: SourceInfo,
    stream: Option<SCStream>,
    rx: Receiver<AudioPacket>,
    stats: Arc<Mutex<CaptureStats>>,
    finished: bool,
}

impl ScreenCaptureAudio {
    pub fn open(source_id: Option<&str>, clock: PipelineClock) -> Result<Self, CaptureError> {
        let content = shareable()?;
        let displays = content.displays();
        let display = match source_id {
            Some(id) => displays.iter().find(|d| d.display_id().to_string() == id),
            None => displays.first(),
        }
        .ok_or_else(|| CaptureError::NoSource("no display available for system audio capture".into()))?;
        let info = SourceInfo {
            id: display.display_id().to_string(),
            label: "System audio (all apps)".into(),
            kind: SourceKind::SystemLoopback,
            is_default: source_id.is_none(),
        };
        let filter = SCContentFilter::create()
            .with_display(display)
            .with_excluding_windows(&[])
            .build()
            .map_err(|e| CaptureError::Device(format!("content filter: {e:?}")))?;
        let config = SCStreamConfiguration::new()
            .with_width(2)
            .with_height(2)
            .with_captures_audio(true)
            .with_excludes_current_process_audio(true)
            .with_sample_rate(SAMPLE_RATE as i32)
            .with_channel_count(CHANNELS as i32);
        let (tx, rx) = sync_channel(256);
        let stats = Arc::new(Mutex::new(CaptureStats::default()));
        let mut stream = SCStream::new(&filter, &config).map_err(|e| CaptureError::Device(format!("stream: {e:?}")))?;
        stream
            .add_output_handler(AudioHandler { tx, clock, stats: stats.clone(), seq: Mutex::new(0) }, SCStreamOutputType::Audio)
            .map_err(|e| CaptureError::Device(format!("audio handler: {e:?}")))?;
        stream.start_capture().map_err(|e| CaptureError::PermissionDenied(format!("{PERMISSION_HELP} ({e:?})")))?;
        Ok(Self { info, stream: Some(stream), rx, stats, finished: false })
    }
}

impl CaptureStream for ScreenCaptureAudio {
    fn source(&self) -> &SourceInfo {
        &self.info
    }

    fn next_packet(&mut self, timeout: Duration) -> Result<Option<AudioPacket>, CaptureError> {
        if self.finished {
            return Ok(None);
        }
        match self.rx.recv_timeout(timeout) {
            Ok(p) => Ok(Some(p)),
            Err(RecvTimeoutError::Timeout) => Ok(None),
            Err(RecvTimeoutError::Disconnected) => {
                self.finished = true;
                Ok(None)
            }
        }
    }

    fn is_finished(&self) -> bool {
        self.finished
    }

    fn stats(&self) -> CaptureStats {
        *self.stats.lock().unwrap()
    }

    fn stop(&mut self) {
        if let Some(s) = self.stream.take() {
            let _ = s.stop_capture();
        }
        self.finished = true;
    }
}

impl Drop for ScreenCaptureAudio {
    fn drop(&mut self) {
        self.stop();
    }
}
