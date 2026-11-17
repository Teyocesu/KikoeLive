//! Translation prompting, context window and output sanitising.
//!
//! The LLM worker is a generic text generator; everything language-specific is
//! here so it is testable and identical on every platform.
//!
//! Rules enforced by the prompt and checked on the output:
//! * only the current line is translated (context is for understanding only),
//! * subject, negation and tense are preserved, nothing is invented,
//! * no notes, explanations, quotes, labels or reasoning in the subtitle.

use std::collections::VecDeque;

use serde::{Deserialize, Serialize};

use crate::lang::TargetLanguage;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PromptStyle {
    /// ChatML with an empty think block: Qwen3 / Qwen3.5 non-thinking mode.
    ChatMlNoThink,
}

const GLOSSARY: &str = "兎田ぺこら = Usada Pekora; ぺこら = Pekora; ぺこ (sentence-ending tic) = peko; \
宝鐘マリン = Houshou Marine; マリン船長 / 船長 = Captain Marine; ホロライブ = hololive; \
スパチャ = superchat; 配信 = stream; コラボ = collab; 先輩 = senpai; 野うさぎ = nousagi (Pekora's fans)";

/// How a target language is produced.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Route {
    /// Japanese -> target in one step.
    Direct,
    /// Japanese -> English draft, then Japanese (+ English draft as a hint) -> target.
    /// The Japanese line stays the source of truth; see EVIDENCE.md for why this
    /// is used for Spanish.
    EnglishAssisted,
}

fn system_prompt(lang: TargetLanguage, assisted: bool) -> String {
    let (target, style) = match lang {
        TargetLanguage::En => ("English", "Natural, casual English, short enough to read as a subtitle."),
        TargetLanguage::Es => (
            "Spanish",
            "Natural, casual, neutral Latin American Spanish (use tú/ustedes, never vosotros), short enough to read as a subtitle. \
Use correct Spanish forms, especially negative commands (¡No te preocupes!, ¡No me dejen solo!).",
        ),
    };
    let draft = if assisted {
        "\nThe user message also contains \"English draft:\", a machine translation of the same line that may contain mistakes. \
The Japanese line is the source of truth: use the draft only to help you understand it, and follow the Japanese wherever they disagree."
    } else {
        ""
    };
    format!(
        "You translate live Japanese speech from streams (VTubers, gaming) into {target} subtitles.\n\
Translate ONLY the text after \"Line:\". Lines under \"Context:\" were said just before; use them only to understand the line and never translate or repeat them.{draft}\n\
Rules:\n\
- Keep the exact meaning: who does what, negation (ない, ません, じゃない, ず, わけじゃない), questions, and tense.\n\
- Japanese often leaves the subject out. In a stream it is usually the speaker (\"I\"); use \"you\" or \"he/she\" only when the line says so or clearly addresses someone.\n\
- Keep every person, place, object, number and name that the line mentions; never replace one with something else.\n\
- Translate set phrases and interjections by their meaning, not word by word (e.g. マジで？ = \"Seriously?\", なるほど = \"I see\").\n\
- Do not add information, explanations, notes, or reactions that are not in the line.\n\
- If the line is an unfinished fragment, translate it as a fragment.\n\
- {style}\n\
- Streamers often call themselves by name (ぺこら = Pekora); keep the name, do not turn it into \"I\".\n\
- Names and terms: {GLOSSARY}.\n\
Output only the {target} subtitle on a single line."
    )
}

/// (Japanese user turn, English answer, Spanish answer). Deliberately different
/// from every line in eval/corpus so the evaluation is not contaminated.
const EXAMPLES: [(&str, &str, &str); 5] = [
    ("Line: 今日はマイクラやるぺこ！", "We're playing Minecraft today, peko!", "¡Hoy jugamos Minecraft, peko!"),
    (
        "Context:\nこのボス、強くない？\nLine: いや、そんなの聞いてないって。",
        "No way, nobody told me about that.",
        "No, nadie me dijo nada de eso.",
    ),
    ("Line: え、誰も来てないの？", "Wait, nobody's here?", "¿Eh, no vino nadie?"),
    ("Line: 昨日ちょっと寝坊しちゃった。", "I overslept a little yesterday.", "Ayer me dormí un poco."),
    ("Line: うわっ！", "Whoa!", "¡Uy!"),
];

pub fn user_message(context: &[String], line: &str) -> String {
    let mut s = String::new();
    if !context.is_empty() {
        s.push_str("Context:\n");
        for c in context {
            s.push_str(c);
            s.push('\n');
        }
    }
    s.push_str("Line: ");
    s.push_str(line);
    s
}

/// A rendered prompt split into the language-constant prefix (cacheable by the
/// worker) and the per-line suffix.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RenderedPrompt {
    pub prefix: String,
    pub suffix: String,
    pub max_tokens: u32,
}

impl RenderedPrompt {
    pub fn full(&self) -> String {
        format!("{}{}", self.prefix, self.suffix)
    }
}

fn chatml(system: String, turns: Vec<(String, String)>, user: String, line: &str) -> RenderedPrompt {
    let mut prefix = format!("<|im_start|>system\n{system}<|im_end|>\n");
    for (u, a) in turns {
        prefix.push_str(&format!("<|im_start|>user\n{u}<|im_end|>\n<|im_start|>assistant\n{a}<|im_end|>\n"));
    }
    let suffix = format!("<|im_start|>user\n{user}<|im_end|>\n<|im_start|>assistant\n<think>\n\n</think>\n\n");
    let chars = line.chars().count() as u32;
    RenderedPrompt { prefix, suffix, max_tokens: (chars * 3).clamp(24, 160) }
}

/// Direct Japanese -> `lang` prompt.
pub fn render_prompt(style: PromptStyle, lang: TargetLanguage, context: &[String], line: &str) -> RenderedPrompt {
    match style {
        PromptStyle::ChatMlNoThink => {
            let turns = EXAMPLES
                .iter()
                .map(|(u, en, es)| (u.to_string(), if lang == TargetLanguage::En { en } else { es }.to_string()))
                .collect();
            chatml(system_prompt(lang, false), turns, user_message(context, line), line)
        }
    }
}

/// Japanese (+ English draft) -> `lang` prompt for [`Route::EnglishAssisted`].
pub fn render_assisted_prompt(style: PromptStyle, lang: TargetLanguage, context: &[String], line: &str, draft: &str) -> RenderedPrompt {
    match style {
        PromptStyle::ChatMlNoThink => {
            let turns = EXAMPLES
                .iter()
                .map(|(u, en, es)| (format!("{u}\nEnglish draft: {en}"), if lang == TargetLanguage::En { en } else { es }.to_string()))
                .collect();
            chatml(system_prompt(lang, true), turns, format!("{}\nEnglish draft: {draft}", user_message(context, line)), line)
        }
    }
}

/// Strings that end generation for ChatML models.
pub fn stop_strings(style: PromptStyle) -> Vec<String> {
    match style {
        PromptStyle::ChatMlNoThink => vec!["<|im_end|>".into(), "<|im_start|>".into(), "\nLine:".into()],
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SanitizeIssue {
    Empty,
    ReasoningLeak,
    UntranslatedJapanese,
    Commentary,
    /// The model answered with one of the few-shot example outputs instead of translating.
    ExampleEcho,
    /// Far too short for the source line (e.g. a long sentence rendered as "Whoa!").
    TooShort,
}

fn is_cjk(c: char) -> bool {
    matches!(c as u32,
        0x3040..=0x30FF | // hiragana + katakana
        0x3400..=0x4DBF | 0x4E00..=0x9FFF | // kanji
        0xFF66..=0xFF9D)
}

const LABELS: &[&str] = &[
    "line:", "translation:", "english:", "spanish:", "español:", "espanol:", "subtitle:", "subtítulo:",
    "traducción:", "traduccion:", "output:", "answer:", "respuesta:",
];

/// Clean raw model output into a single subtitle line.
pub fn sanitize_output(raw: &str) -> Result<String, SanitizeIssue> {
    let mut s = raw.to_string();
    // Remove complete think blocks; an unterminated one means reasoning leaked.
    while let Some(start) = s.find("<think>") {
        match s[start..].find("</think>") {
            Some(end) => s.replace_range(start..start + end + "</think>".len(), ""),
            None => return Err(SanitizeIssue::ReasoningLeak),
        }
    }
    if s.contains("</think>") {
        // reasoning before a closing tag without an opening one
        s = s.rsplit("</think>").next().unwrap_or("").to_string();
    }
    for tok in ["<|im_end|>", "<|im_start|>", "<|endoftext|>"] {
        s = s.replace(tok, "\n");
    }
    let line = s.lines().map(str::trim).find(|l| !l.is_empty()).unwrap_or("").to_string();
    let mut line = line;
    loop {
        let lower = line.to_lowercase();
        let Some(label) = LABELS.iter().find(|l| lower.starts_with(**l)) else { break };
        line = line[label.len()..].trim_start().to_string();
    }
    let lower = line.to_lowercase();
    if lower.starts_with("note") || lower.starts_with("nota") || lower.starts_with("(note") || lower.starts_with("(nota") {
        return Err(SanitizeIssue::Commentary);
    }
    // Drop a trailing translator's note in parentheses.
    if let Some(open) = line.rfind(" (") {
        let tail = line[open..].to_lowercase();
        if line.ends_with(')')
            && ["note", "lit.", "literally", "meaning", "nota", "literalmente", "significa"].iter().any(|k| tail.contains(k))
        {
            line.truncate(open);
        }
    }
    let quotes: &[(char, char)] = &[('"', '"'), ('“', '”'), ('「', '」'), ('\'', '\''), ('«', '»')];
    for (o, c) in quotes {
        if line.chars().count() >= 2 && line.starts_with(*o) && line.ends_with(*c) {
            line = line[o.len_utf8()..line.len() - c.len_utf8()].trim().to_string();
        }
    }
    let line = line.trim().to_string();
    if line.is_empty() {
        return Err(SanitizeIssue::Empty);
    }
    let letters = line.chars().filter(|c| c.is_alphabetic()).count().max(1);
    let cjk = line.chars().filter(|c| is_cjk(*c)).count();
    if cjk >= 3 && cjk * 10 > letters * 3 {
        return Err(SanitizeIssue::UntranslatedJapanese);
    }
    Ok(line)
}

/// Reject outputs that cannot be a translation of `ja` even though they are
/// well-formed: an echo of a few-shot answer, or a drastically truncated line.
/// (Observed: a 84-character FLEURS sentence translated as "¡Uy!".)
pub fn check_plausible(ja: &str, out: &str) -> Result<(), SanitizeIssue> {
    let src_chars = ja.chars().filter(|c| !c.is_whitespace()).count();
    let norm = |s: &str| s.trim().trim_end_matches(['.', '!', '?', '¡', '¿']).to_lowercase();
    // Short interjections legitimately share answers with examples ("Whoa!").
    let echoed = src_chars >= 8 && EXAMPLES.iter().any(|(u, en, es)| {
        let example_line = u.rsplit("Line: ").next().unwrap_or("");
        example_line != ja && (norm(out) == norm(en) || norm(out) == norm(es))
    });
    if echoed {
        return Err(SanitizeIssue::ExampleEcho);
    }
    let out_letters = out.chars().filter(|c| c.is_alphanumeric()).count();
    if src_chars >= 16 && out_letters * 2 < src_chars {
        return Err(SanitizeIssue::TooShort);
    }
    Ok(())
}

/// Recent Japanese lines used as context for the next translation.
/// Lines are pushed as soon as ASR produces them (possibly ahead of translation),
/// so selection only takes lines that ended before the current one started.
pub struct ContextWindow {
    lines: VecDeque<(u64, String)>,
    max_lines: usize,
    max_gap_ms: u64,
}

const CONTEXT_KEEP: usize = 8;

impl ContextWindow {
    pub fn new(max_lines: usize, max_gap_ms: u64) -> Self {
        Self { lines: VecDeque::new(), max_lines, max_gap_ms }
    }

    /// Context for a line whose voiced audio starts at `start_ms` (stream time).
    pub fn context_for(&self, start_ms: u64) -> Vec<String> {
        let eligible: Vec<&(u64, String)> = self
            .lines
            .iter()
            .filter(|(end, _)| *end <= start_ms && start_ms - *end <= self.max_gap_ms)
            .collect();
        let skip = eligible.len().saturating_sub(self.max_lines);
        eligible.into_iter().skip(skip).map(|(_, t)| t.clone()).collect()
    }

    pub fn push(&mut self, end_ms: u64, ja: String) {
        if self.max_lines == 0 {
            return;
        }
        self.lines.push_back((end_ms, ja));
        while self.lines.len() > CONTEXT_KEEP {
            self.lines.pop_front();
        }
    }

    pub fn clear(&mut self) {
        self.lines.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prompt_prefix_is_constant_per_language_and_suffix_holds_line() {
        let a = render_prompt(PromptStyle::ChatMlNoThink, TargetLanguage::En, &[], "待って");
        let b = render_prompt(PromptStyle::ChatMlNoThink, TargetLanguage::En, &["前の行".into()], "行かないで");
        assert_eq!(a.prefix, b.prefix);
        assert!(b.suffix.contains("Context:\n前の行\nLine: 行かないで"));
        assert!(a.suffix.ends_with("<think>\n\n</think>\n\n"), "thinking must be disabled");
        let es = render_prompt(PromptStyle::ChatMlNoThink, TargetLanguage::Es, &[], "待って");
        assert_ne!(es.prefix, a.prefix);
        assert!(es.prefix.contains("Latin American Spanish"));
    }

    #[test]
    fn assisted_prompt_keeps_japanese_as_source_and_draft_in_suffix() {
        let p = render_assisted_prompt(PromptStyle::ChatMlNoThink, TargetLanguage::Es, &[], "行かないで", "Don't go");
        assert!(p.prefix.contains("source of truth"));
        assert!(p.suffix.contains("Line: 行かないで
English draft: Don't go"));
        let q = render_assisted_prompt(PromptStyle::ChatMlNoThink, TargetLanguage::Es, &[], "待って", "Wait");
        assert_eq!(p.prefix, q.prefix, "assisted prefix must be cacheable");
    }

    #[test]
    fn sanitize_strips_reasoning_labels_quotes_and_notes() {
        assert_eq!(sanitize_output("<think>\nhmm\n</think>\n\nI won't go.").unwrap(), "I won't go.");
        assert_eq!(sanitize_output("Translation: \"Don't laugh!\"").unwrap(), "Don't laugh!");
        assert_eq!(sanitize_output("Wait, nobody's here? (Note: literally 'nobody came')").unwrap(), "Wait, nobody's here?");
        assert_eq!(sanitize_output("No lo sabía.\nNota: el hablante...").unwrap(), "No lo sabía.");
        assert_eq!(sanitize_output("<think>unfinished reasoning"), Err(SanitizeIssue::ReasoningLeak));
        assert_eq!(sanitize_output("Note: this is a greeting"), Err(SanitizeIssue::Commentary));
        assert_eq!(sanitize_output("   \n "), Err(SanitizeIssue::Empty));
        assert_eq!(sanitize_output("全然怖くないし"), Err(SanitizeIssue::UntranslatedJapanese));
        // a single kanji name inside an English line is acceptable
        assert_eq!(sanitize_output("That's Pekora's (ぺこら) fault").unwrap(), "That's Pekora's (ぺこら) fault");
    }

    #[test]
    fn implausible_outputs_are_rejected() {
        let long = "彼女がこの結論に至ったのは、避妊薬が医学的に必要であると考えるよう促す多くの肯定的な意見や励ましが寄せられたためです。";
        assert_eq!(check_plausible(long, "¡Uy!"), Err(SanitizeIssue::ExampleEcho));
        assert_eq!(check_plausible(long, "She agreed."), Err(SanitizeIssue::TooShort));
        assert!(check_plausible("うわっ！", "Whoa!").is_ok(), "the example's own line may produce the example answer");
        assert!(check_plausible("やった！", "Yay!").is_ok(), "short lines may have short translations");
        assert!(check_plausible("うわあ！", "Whoa!").is_ok(), "an interjection may match an example answer");
        assert!(check_plausible("まだ行かないで、ちょっと待って。", "Don't go yet, wait a second.").is_ok());
    }

    #[test]
    fn context_window_is_bounded_and_age_limited() {
        let mut w = ContextWindow::new(2, 10_000);
        w.push(1_000, "一".into());
        w.push(2_000, "二".into());
        w.push(3_000, "三".into());
        assert_eq!(w.context_for(4_000), vec!["二".to_string(), "三".to_string()]);
        assert_eq!(w.context_for(12_500), vec!["三".to_string()]);
        assert!(w.context_for(60_000).is_empty());
    }

    #[test]
    fn context_never_includes_the_current_or_later_lines() {
        let mut w = ContextWindow::new(2, 10_000);
        w.push(1_000, "前".into());
        w.push(5_000, "今".into()); // the line being translated (ends at 5 s)
        w.push(7_000, "後".into()); // ASR ran ahead of translation
        // current line voiced from 3.5 s to 5 s
        assert_eq!(w.context_for(3_500), vec!["前".to_string()]);
    }
}
