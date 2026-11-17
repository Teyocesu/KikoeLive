//! Target language selection, shared between the UI thread and the pipeline.
//!
//! Every change bumps an `epoch`. Translation jobs carry the epoch they were
//! started under; the coordinator refuses to publish a result whose epoch is no
//! longer current, which is what prevents stale / mixed-language subtitles after
//! a switch. A switch also raises a cancel flag the translation worker polls.

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU8, AtomicU64, Ordering};

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TargetLanguage {
    En,
    Es,
}

impl TargetLanguage {
    pub const ALL: [TargetLanguage; 2] = [TargetLanguage::En, TargetLanguage::Es];

    pub fn code(self) -> &'static str {
        match self {
            TargetLanguage::En => "en",
            TargetLanguage::Es => "es",
        }
    }

    pub fn pair(self) -> &'static str {
        match self {
            TargetLanguage::En => "ja-en",
            TargetLanguage::Es => "ja-es",
        }
    }

    pub fn native_name(self) -> &'static str {
        match self {
            TargetLanguage::En => "English",
            TargetLanguage::Es => "Español",
        }
    }

    pub fn parse(s: &str) -> Option<Self> {
        match s.trim().to_ascii_lowercase().as_str() {
            "en" | "english" | "ja-en" => Some(TargetLanguage::En),
            "es" | "spanish" | "español" | "espanol" | "ja-es" => Some(TargetLanguage::Es),
            _ => None,
        }
    }

    fn to_u8(self) -> u8 {
        match self {
            TargetLanguage::En => 0,
            TargetLanguage::Es => 1,
        }
    }

    fn from_u8(v: u8) -> Self {
        if v == 1 { TargetLanguage::Es } else { TargetLanguage::En }
    }
}

#[derive(Debug)]
pub struct LanguageControl {
    lang: AtomicU8,
    epoch: AtomicU64,
    switch_cancel: Arc<AtomicBool>,
}

impl LanguageControl {
    pub fn new(initial: TargetLanguage) -> Arc<Self> {
        Arc::new(Self {
            lang: AtomicU8::new(initial.to_u8()),
            epoch: AtomicU64::new(1),
            switch_cancel: Arc::new(AtomicBool::new(false)),
        })
    }

    /// Current (language, epoch). Read both through this to get a consistent pair.
    pub fn current(&self) -> (TargetLanguage, u64) {
        loop {
            let e1 = self.epoch.load(Ordering::Acquire);
            let l = TargetLanguage::from_u8(self.lang.load(Ordering::Acquire));
            let e2 = self.epoch.load(Ordering::Acquire);
            if e1 == e2 {
                return (l, e1);
            }
        }
    }

    pub fn epoch(&self) -> u64 {
        self.epoch.load(Ordering::Acquire)
    }

    /// Select a language. Returns the new epoch, or `None` if it was already selected.
    pub fn set(&self, lang: TargetLanguage) -> Option<u64> {
        let prev = self.lang.swap(lang.to_u8(), Ordering::AcqRel);
        if prev == lang.to_u8() {
            return None;
        }
        let e = self.epoch.fetch_add(1, Ordering::AcqRel) + 1;
        self.switch_cancel.store(true, Ordering::Release);
        Some(e)
    }

    /// Flag polled by an in-flight translation; set on every switch.
    pub fn switch_cancel_flag(&self) -> Arc<AtomicBool> {
        self.switch_cancel.clone()
    }
}

const EN_WORDS: &[&str] = &[
    "the", "a", "an", "is", "are", "was", "i", "you", "it", "that", "this", "to", "of", "and", "in", "on", "what",
    "don't", "i'm", "it's", "not", "my", "me", "we", "they", "have", "do", "so", "just", "oh", "wait", "okay", "but",
    "with", "for", "be", "can", "will", "gonna", "there", "your", "again", "thanks", "who", "why", "did", "didn't",
];
const ES_WORDS: &[&str] = &[
    "el", "la", "los", "las", "un", "una", "es", "son", "que", "de", "y", "en", "yo", "tú", "me", "mi", "te", "se",
    "lo", "por", "para", "con", "pero", "qué", "está", "estoy", "eso", "esto", "muy", "ya", "todavía", "también",
    "gracias", "nada", "otra", "vez", "quién", "dijo", "hay", "vamos", "voy", "puedo", "tengo", "nunca", "más",
];

/// Rough guess of whether a subtitle is English or Spanish. Used only as a
/// diagnostic guard against the model answering in the wrong language; `None`
/// when the text is too short or ambiguous to tell.
pub fn guess_latin_language(text: &str) -> Option<TargetLanguage> {
    let lower = text.to_lowercase();
    let mut en = 0i32;
    let mut es = 0i32;
    if lower.contains(['¿', '¡', 'ñ', 'á', 'é', 'í', 'ó', 'ú']) {
        es += 2;
    }
    for w in lower.split(|c: char| !(c.is_alphanumeric() || c == '\'' || c == '’')) {
        if w.is_empty() {
            continue;
        }
        let w = w.replace('’', "'");
        if EN_WORDS.contains(&w.as_str()) {
            en += 1;
        }
        if ES_WORDS.contains(&w.as_str()) {
            es += 1;
        }
    }
    if en >= 2 && en >= 2 * es.max(1) {
        Some(TargetLanguage::En)
    } else if es >= 2 && es >= 2 * en.max(1) {
        Some(TargetLanguage::Es)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn switching_bumps_epoch_and_raises_cancel() {
        let c = LanguageControl::new(TargetLanguage::En);
        let (l0, e0) = c.current();
        assert_eq!(l0, TargetLanguage::En);
        assert_eq!(c.set(TargetLanguage::En), None, "same language is a no-op");
        let e1 = c.set(TargetLanguage::Es).unwrap();
        assert!(e1 > e0);
        assert!(c.switch_cancel_flag().load(Ordering::Acquire));
        assert_eq!(c.current(), (TargetLanguage::Es, e1));
    }

    #[test]
    fn guesses_language_of_typical_subtitles() {
        assert_eq!(guess_latin_language("I'm never doing this again."), Some(TargetLanguage::En));
        assert_eq!(guess_latin_language("No lo vuelvo a hacer nunca más."), Some(TargetLanguage::Es));
        assert_eq!(guess_latin_language("¡Gracias por el superchat!"), Some(TargetLanguage::Es));
        assert_eq!(guess_latin_language("Pekora!"), None);
    }
}
