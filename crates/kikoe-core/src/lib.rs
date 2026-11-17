//! KikoeLive core: everything that is the same on macOS and Windows.
//!
//! Pipeline (one direction, bounded at every hop):
//! SystemAudioCapture -> AudioNormalization (downmix + resample to 16 kHz mono)
//! -> VoiceActivityDetection -> SpeechSegmentation -> JapaneseASR
//! -> ContextManagement -> TargetLanguageTranslation -> SubtitleCoordinator -> UI.
//!
//! Platform adapters (capture, inference engines, windows) live in other crates and
//! plug in through the traits in [`pipeline`] and [`audio`].

pub mod audio;
pub mod clock;
pub mod coordinator;
pub mod filters;
pub mod lang;
pub mod metrics;
pub mod models;
pub mod pipeline;
pub mod protocol;
pub mod queue;
pub mod resample;
pub mod segmenter;
pub mod translate;
pub mod worker;
