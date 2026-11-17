//! Local settings (one JSON file in the user's config directory). Nothing here
//! leaves the machine. Transcripts and audio are never stored.

use std::collections::HashMap;
use std::path::PathBuf;

use kikoe_core::lang::TargetLanguage;
use kikoe_core::models::{DEFAULT_ASR_ID, VerifiedRecord};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct OverlaySettings {
    pub visible: bool,
    /// Background opacity 0.0 - 1.0.
    pub opacity: f32,
    /// Translated line size in px; the Japanese line is drawn at 60 %.
    pub font_size: u32,
}

impl Default for OverlaySettings {
    fn default() -> Self {
        Self { visible: true, opacity: 0.72, font_size: 30 }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Settings {
    pub target_language: TargetLanguage,
    pub show_japanese: bool,
    /// `None` follows the OS default output.
    pub source_id: Option<String>,
    pub asr_model: String,
    pub use_gpu: bool,
    pub overlay: OverlaySettings,
    /// Model files the user pointed at (already on disk elsewhere); used only after verification.
    pub model_paths: HashMap<String, String>,
    pub verified: Vec<VerifiedRecord>,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            target_language: TargetLanguage::En,
            show_japanese: true,
            source_id: None,
            asr_model: DEFAULT_ASR_ID.into(),
            use_gpu: true,
            overlay: OverlaySettings::default(),
            model_paths: HashMap::new(),
            verified: Vec::new(),
        }
    }
}

pub fn app_dir() -> PathBuf {
    dirs::config_dir().unwrap_or_else(std::env::temp_dir).join("KikoeLive")
}

fn path() -> PathBuf {
    app_dir().join("settings.json")
}

impl Settings {
    pub fn load() -> Self {
        let p = path();
        match std::fs::read_to_string(&p) {
            Ok(text) => match serde_json::from_str(&text) {
                Ok(s) => s,
                Err(e) => {
                    log::warn!("settings unreadable ({e}); keeping a copy and using defaults");
                    let _ = std::fs::copy(&p, p.with_extension("json.bad"));
                    Settings::default()
                }
            },
            Err(_) => Settings::default(),
        }
    }

    pub fn save(&self) -> Result<(), String> {
        let p = path();
        std::fs::create_dir_all(p.parent().unwrap()).map_err(|e| e.to_string())?;
        let tmp = p.with_extension("json.tmp");
        std::fs::write(&tmp, serde_json::to_string_pretty(self).map_err(|e| e.to_string())?).map_err(|e| e.to_string())?;
        std::fs::rename(&tmp, &p).map_err(|e| e.to_string())
    }
}
