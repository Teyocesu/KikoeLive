//! Filters applied to Japanese ASR output before anything reaches translation.
//!
//! Phase 1 found three failure families that must not reach the subtitle:
//! 1. nonverbal / bracketed annotations Whisper invents for music or noise,
//! 2. canned "closing" phrases Whisper hallucinates on silence or BGM,
//! 3. decoding loops (the same unit repeated many times).
//! Each rule here is conservative: a phrase a streamer could really say is only
//! dropped when the acoustic evidence is weak as well.

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize)]
pub struct AsrEvidence {
    /// Whisper's no-speech probability for the decoded segment (0..1).
    pub no_speech_prob: f32,
    /// Mean token probability of the decoded text (0..1).
    pub avg_token_prob: f32,
    /// Mean VAD probability over voiced frames (0..1).
    pub vad_mean_prob: f32,
    /// Voiced milliseconds in the segment.
    pub voiced_ms: u32,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "verdict", content = "detail")]
pub enum FilterVerdict {
    Keep,
    /// Text was kept but modified (e.g. a loop collapsed).
    Cleaned(String),
    Drop(DropReason),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DropReason {
    Empty,
    Nonverbal,
    LikelyHallucination,
    LowConfidence,
    RepetitionLoop,
    DuplicateOfPrevious,
}

/// Phrases Whisper is known to produce on silence / music in Japanese.
const HALLUCINATION_PHRASES: &[&str] = &[
    "ご視聴ありがとうございました",
    "ご清聴ありがとうございました",
    "最後までご視聴いただきありがとうございました",
    "チャンネル登録をお願いします",
    "チャンネル登録よろしくお願いします",
    "高評価とチャンネル登録",
    "字幕は自動生成されています",
    "字幕作成",
    "提供",
    "次回もお楽しみに",
    "おやすみなさい",
    "エンディング",
];

const NONVERBAL_WORDS: &[&str] = &[
    "音楽", "拍手", "笑", "笑い", "BGM", "効果音", "歓声", "咳", "ため息", "無音", "雑音", "music", "Music",
    "MUSIC", "applause", "laughs", "laughter",
];

pub fn normalize_ja(text: &str) -> String {
    let t = text.trim();
    let mut out = String::with_capacity(t.len());
    let mut prev_space = false;
    for c in t.chars() {
        let c = if c == '\u{3000}' { ' ' } else { c };
        if c.is_whitespace() {
            if !prev_space {
                out.push(' ');
            }
            prev_space = true;
        } else {
            out.push(c);
            prev_space = false;
        }
    }
    out.trim().to_string()
}

fn is_punct_or_symbol(c: char) -> bool {
    c.is_ascii_punctuation()
        || matches!(c, '。' | '、' | '！' | '？' | '・' | '…' | '「' | '」' | '『' | '』' | '（' | '）' | '【' | '】' | '〜' | '～' | 'ー' | '♪' | '♫' | '♬' | '＊' | '※' | '　')
        || c.is_whitespace()
}

/// Text with every bracketed span removed.
fn strip_bracketed(text: &str) -> String {
    let mut out = String::new();
    let mut depth = 0i32;
    for c in text.chars() {
        match c {
            '(' | '（' | '[' | '［' | '【' | '〔' | '<' | '＜' => depth += 1,
            ')' | '）' | ']' | '］' | '】' | '〕' | '>' | '＞' => depth = (depth - 1).max(0),
            _ if depth == 0 => out.push(c),
            _ => {}
        }
    }
    out
}

pub fn is_nonverbal(text: &str) -> bool {
    let t = text.trim();
    if t.is_empty() {
        return true;
    }
    if t.chars().all(is_punct_or_symbol) {
        return true;
    }
    let outside = strip_bracketed(t);
    if outside.chars().all(is_punct_or_symbol) {
        return true; // e.g. "(音楽)" "【拍手】" "♪～"
    }
    let bare: String = t.chars().filter(|c| !is_punct_or_symbol(*c)).collect();
    NONVERBAL_WORDS.iter().any(|w| bare == *w)
}

fn matches_hallucination(text: &str) -> bool {
    let bare: String = text.chars().filter(|c| !is_punct_or_symbol(*c)).collect();
    if bare.is_empty() {
        return false;
    }
    HALLUCINATION_PHRASES.iter().any(|p| bare == *p || (bare.contains(p) && bare.chars().count() <= p.chars().count() + 6))
}

/// Collapse a unit repeated more than `max_repeats` times in a row.
/// Returns the cleaned text and whether a runaway loop (dominating the text) was found.
pub fn collapse_repetitions(text: &str, max_repeats: usize) -> (String, bool) {
    let chars: Vec<char> = text.chars().collect();
    let n = chars.len();
    let mut out: Vec<char> = Vec::with_capacity(n);
    let mut i = 0;
    let mut longest_loop = 0usize;
    while i < n {
        let mut collapsed = false;
        for unit in 1..=12usize {
            if i + unit * (max_repeats + 1) > n {
                break;
            }
            let pat = &chars[i..i + unit];
            if pat.iter().all(|c| is_punct_or_symbol(*c)) {
                continue;
            }
            let mut reps = 1;
            while i + (reps + 1) * unit <= n && &chars[i + reps * unit..i + (reps + 1) * unit] == pat {
                reps += 1;
            }
            if reps > max_repeats {
                for _ in 0..max_repeats {
                    out.extend_from_slice(pat);
                }
                longest_loop = longest_loop.max(reps * unit);
                i += reps * unit;
                collapsed = true;
                break;
            }
        }
        if !collapsed {
            out.push(chars[i]);
            i += 1;
        }
    }
    let runaway = n >= 16 && longest_loop as f32 >= n as f32 * 0.6;
    (out.into_iter().collect(), runaway)
}

pub struct TranscriptFilter {
    last_text: Option<String>,
    last_end_ms: u64,
}

impl Default for TranscriptFilter {
    fn default() -> Self {
        Self::new()
    }
}

impl TranscriptFilter {
    pub fn new() -> Self {
        Self { last_text: None, last_end_ms: 0 }
    }

    pub fn reset(&mut self) {
        self.last_text = None;
        self.last_end_ms = 0;
    }

    /// `start_ms`/`end_ms` are stream times of the voiced region of this segment.
    pub fn check(&mut self, raw: &str, ev: &AsrEvidence, start_ms: u64, end_ms: u64) -> FilterVerdict {
        let text = normalize_ja(raw);
        if text.is_empty() {
            return FilterVerdict::Drop(DropReason::Empty);
        }
        if is_nonverbal(&text) {
            return FilterVerdict::Drop(DropReason::Nonverbal);
        }
        let weak = ev.no_speech_prob > 0.15 || ev.avg_token_prob < 0.55 || ev.vad_mean_prob < 0.55;
        if matches_hallucination(&text) && weak {
            return FilterVerdict::Drop(DropReason::LikelyHallucination);
        }
        if ev.no_speech_prob > 0.6 && ev.avg_token_prob < 0.4 {
            return FilterVerdict::Drop(DropReason::LowConfidence);
        }
        let (cleaned, runaway) = collapse_repetitions(&text, 3);
        if runaway && weak {
            return FilterVerdict::Drop(DropReason::RepetitionLoop);
        }
        // Strip bracketed annotations embedded in real speech: "(笑)そうだね" -> "そうだね"
        let cleaned = {
            let s = normalize_ja(&strip_bracketed(&cleaned));
            if s.is_empty() { cleaned } else { s }
        };
        // Exact repeat of the previous line almost immediately, on a short clip:
        // treat as a duplicate decode, not as the speaker repeating themselves.
        if let Some(prev) = &self.last_text {
            let gap = start_ms.saturating_sub(self.last_end_ms);
            if *prev == cleaned && gap < 1_000 && ev.voiced_ms < 1_200 && weak {
                return FilterVerdict::Drop(DropReason::DuplicateOfPrevious);
            }
        }
        self.last_text = Some(cleaned.clone());
        self.last_end_ms = end_ms;
        if cleaned != text { FilterVerdict::Cleaned(cleaned) } else { FilterVerdict::Keep }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn strong() -> AsrEvidence {
        AsrEvidence { no_speech_prob: 0.01, avg_token_prob: 0.9, vad_mean_prob: 0.9, voiced_ms: 1500 }
    }

    fn weak() -> AsrEvidence {
        AsrEvidence { no_speech_prob: 0.4, avg_token_prob: 0.5, vad_mean_prob: 0.5, voiced_ms: 600 }
    }

    #[test]
    fn nonverbal_annotations_are_dropped() {
        let mut f = TranscriptFilter::new();
        for t in ["(音楽)", "【拍手】", "♪～", "（笑）", "...", "[Music]"] {
            assert_eq!(f.check(t, &strong(), 0, 1000), FilterVerdict::Drop(DropReason::Nonverbal), "{t}");
        }
    }

    #[test]
    fn canned_phrase_dropped_only_with_weak_evidence() {
        let mut f = TranscriptFilter::new();
        assert_eq!(
            f.check("ご視聴ありがとうございました。", &weak(), 0, 1000),
            FilterVerdict::Drop(DropReason::LikelyHallucination)
        );
        // A streamer genuinely asking for subscriptions, clearly voiced, is kept.
        assert_eq!(f.check("チャンネル登録よろしくお願いします！", &strong(), 0, 1000), FilterVerdict::Keep);
    }

    #[test]
    fn loops_are_collapsed_and_runaways_dropped() {
        let (c, runaway) = collapse_repetitions("やばいやばいやばいやばいやばいやばい", 3);
        assert_eq!(c, "やばいやばいやばい");
        assert!(runaway);
        let mut f = TranscriptFilter::new();
        assert_eq!(
            f.check("ありがとうありがとうありがとうありがとうありがとうありがとう", &weak(), 0, 1000),
            FilterVerdict::Drop(DropReason::RepetitionLoop)
        );
        // Legit triple repetition untouched.
        assert_eq!(f.check("死ぬ死ぬ死ぬ！", &strong(), 2000, 3000), FilterVerdict::Keep);
    }

    #[test]
    fn negation_text_is_never_altered() {
        let mut f = TranscriptFilter::new();
        for t in ["全然怖くないし。", "まだ行かないで", "もう二度とやらない。"] {
            assert_eq!(f.check(t, &strong(), 0, 1000), FilterVerdict::Keep);
        }
    }

    #[test]
    fn embedded_annotation_is_stripped_but_speech_kept() {
        let mut f = TranscriptFilter::new();
        assert_eq!(f.check("(笑)そうだね", &strong(), 0, 1000), FilterVerdict::Cleaned("そうだね".into()));
    }

    #[test]
    fn immediate_weak_duplicate_is_dropped_but_real_repeat_kept() {
        let mut f = TranscriptFilter::new();
        assert_eq!(f.check("待って", &strong(), 0, 800), FilterVerdict::Keep);
        assert_eq!(f.check("待って", &weak(), 900, 1400), FilterVerdict::Drop(DropReason::DuplicateOfPrevious));
        // clearly voiced repeat a few seconds later is a real repeat
        assert_eq!(f.check("待って", &strong(), 5000, 5800), FilterVerdict::Keep);
    }
}
