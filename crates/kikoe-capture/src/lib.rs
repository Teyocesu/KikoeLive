//! System-audio capture behind `kikoe_core::audio::CaptureStream`.
//!
//! * Windows: WASAPI shared-mode loopback on a render endpoint ([`wasapi`]).
//! * macOS: ScreenCaptureKit audio, excluding KikoeLive's own process ([`macos`]).
//! * Files: real-time-paced or as-fast-as-possible playback of decoded PCM, used
//!   only by the evaluation harness ([`file`]).
//!
//! There is deliberately no microphone path anywhere in this crate.

use kikoe_core::audio::{CaptureError, CaptureStream, SourceInfo};
use kikoe_core::clock::PipelineClock;

pub mod file;

#[cfg(windows)]
pub mod wasapi;

#[cfg(target_os = "macos")]
pub mod macos;

/// System-audio sources the user can pick from.
pub fn list_sources() -> Result<Vec<SourceInfo>, CaptureError> {
    #[cfg(windows)]
    {
        wasapi::list_sources()
    }
    #[cfg(target_os = "macos")]
    {
        macos::list_sources()
    }
    #[cfg(not(any(windows, target_os = "macos")))]
    {
        Err(CaptureError::Unsupported("system audio capture is only implemented for Windows and macOS".into()))
    }
}

/// Open system-audio capture. `source_id = None` follows the OS default output.
pub fn open_system_capture(source_id: Option<&str>, clock: PipelineClock) -> Result<Box<dyn CaptureStream>, CaptureError> {
    #[cfg(windows)]
    {
        Ok(Box::new(wasapi::WasapiLoopback::open(source_id, clock)?))
    }
    #[cfg(target_os = "macos")]
    {
        Ok(Box::new(macos::ScreenCaptureAudio::open(source_id, clock)?))
    }
    #[cfg(not(any(windows, target_os = "macos")))]
    {
        let _ = (source_id, clock);
        Err(CaptureError::Unsupported("system audio capture is only implemented for Windows and macOS".into()))
    }
}
