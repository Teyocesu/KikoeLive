//! Text metrics (chrF, CER) and alignment of pipeline output with a truth file.

use std::collections::HashMap;

use kikoe_core::lang::TargetLanguage;
use kikoe_core::metrics::{Summary, summarize};
use kikoe_core::pipeline::{PipelineEvent, PipelineStats, StopReport};
use serde::Serialize;

use crate::Item;

/// Sentence-level chrF (character 1..6-grams, beta = 2, whitespace ignored), 0..100.
pub fn chrf(hyp: &str, reference: &str) -> f64 {
    let h: Vec<char> = hyp.to_lowercase().chars().filter(|c| !c.is_whitespace()).collect();
    let r: Vec<char> = reference.to_lowercase().chars().filter(|c| !c.is_whitespace()).collect();
    if h.is_empty() || r.is_empty() {
        return 0.0;
    }
    let (mut p_sum, mut r_sum, mut n_used) = (0.0, 0.0, 0);
    for n in 1..=6 {
        if h.len() < n || r.len() < n {
            break;
        }
        let (hg, rg) = (grams(&h, n), grams(&r, n));
        let overlap: usize = hg.iter().map(|(g, c)| (*c).min(*rg.get(g).unwrap_or(&0))).sum();
        p_sum += overlap as f64 / (h.len() - n + 1) as f64;
        r_sum += overlap as f64 / (r.len() - n + 1) as f64;
        n_used += 1;
    }
    if n_used == 0 {
        return 0.0;
    }
    let (p, rc) = (p_sum / n_used as f64, r_sum / n_used as f64);
    if p + rc == 0.0 {
        return 0.0;
    }
    let b2 = 4.0;
    100.0 * (1.0 + b2) * p * rc / (b2 * p + rc)
}

fn grams(v: &[char], n: usize) -> HashMap<&[char], usize> {
    let mut m: HashMap<&[char], usize> = HashMap::new();
    for g in v.windows(n) {
        *m.entry(g).or_default() += 1;
    }
    m
}

fn normalize_ja(s: &str) -> Vec<char> {
    s.chars()
        .filter_map(|c| {
            let c = match c as u32 {
                0xFF01..=0xFF5E => char::from_u32(c as u32 - 0xFEE0).unwrap_or(c),
                _ => c,
            };
            if c.is_alphanumeric() { Some(c.to_ascii_lowercase()) } else { None }
        })
        .collect()
}

/// (edit distance, reference length) after removing punctuation/space.
pub fn cer_counts(hyp: &str, reference: &str) -> (usize, usize) {
    let h = normalize_ja(hyp);
    let r = normalize_ja(reference);
    let mut prev: Vec<usize> = (0..=r.len()).collect();
    for i in 1..=h.len() {
        let mut cur = vec![i; r.len() + 1];
        for j in 1..=r.len() {
            let sub = prev[j - 1] + usize::from(h[i - 1] != r[j - 1]);
            cur[j] = sub.min(prev[j] + 1).min(cur[j - 1] + 1);
        }
        prev = cur;
    }
    (prev[r.len()], r.len())
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct UtteranceResult {
    pub id: String,
    pub reference_ja: String,
    pub asr: String,
    pub cer: f64,
    pub lang: Option<String>,
    pub reference: String,
    pub translation: String,
    pub chrf: f64,
    pub segments: Vec<u64>,
    /// true speech start/end (truth file) -> first translation published, ms
    pub start_to_subtitle_ms: Option<f64>,
    pub end_to_subtitle_ms: Option<f64>,
    pub skipped: Vec<String>,
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct Analysis {
    pub utterances: Vec<UtteranceResult>,
    pub unmatched_segments: Vec<(u64, String)>,
    pub corpus_cer: f64,
    pub mean_chrf_en: Option<f64>,
    pub mean_chrf_es: Option<f64>,
    pub latency_start_en: Summary,
    pub latency_end_en: Summary,
    pub latency_start_es: Summary,
    pub latency_end_es: Summary,
    /// Translations published after a switch that were in the old language.
    pub stale_language_after_switch: usize,
    pub switch_events: Vec<String>,
    pub dropped: Vec<String>,
    pub errors: Vec<String>,
    pub t0_offset_ns: i64,
}

/// For live capture the truth timeline starts when playback started, which is
/// not known exactly: pick the offset (0-15 s, 20 ms steps) that places the most
/// transcript midpoints inside truth utterances.
pub fn estimate_offset_ms(events: &[PipelineEvent], truth: &[Item]) -> i64 {
    let mids: Vec<i64> = events
        .iter()
        .filter_map(|e| match e {
            PipelineEvent::Transcript { start_ms, end_ms, .. } => Some(((*start_ms + *end_ms) / 2) as i64),
            _ => None,
        })
        .collect();
    let spans: Vec<(i64, i64)> =
        truth.iter().filter_map(|t| Some(((t.start_s? * 1000.0) as i64, (t.end_s? * 1000.0) as i64))).collect();
    let mut best = (0usize, 0i64);
    for off in (0..15_000).step_by(20) {
        let score = mids.iter().filter(|m| spans.iter().any(|(s, e)| **m >= s + off - 200 && **m <= e + off + 200)).count();
        if score > best.0 {
            best = (score, off);
        }
    }
    best.1
}

/// Match pipeline segments to truth utterances by time overlap (stream time).
/// `offset_ms` shifts the truth timeline (0 for file runs).
pub fn align(events: &[PipelineEvent], truth: &[Item], offset_ms: i64) -> Analysis {
    let mut an = Analysis::default();
    let mut transcripts: Vec<(u64, String, u64, u64, u64)> = Vec::new(); // id, ja, start_ms, end_ms, speech_start_ns
    let mut translations: HashMap<u64, Vec<(TargetLanguage, String, u64, u64)>> = HashMap::new(); // lang, text, published_ns, epoch
    let mut skipped: HashMap<u64, Vec<String>> = HashMap::new();
    let mut offsets: Vec<i64> = Vec::new();
    for e in events {
        match e {
            PipelineEvent::Transcript { seg_id, ja, start_ms, end_ms, timing } => {
                transcripts.push((*seg_id, ja.clone(), *start_ms, *end_ms, timing.speech_start_ns));
                offsets.push(timing.speech_start_ns as i64 - (*start_ms as i64) * 1_000_000);
            }
            PipelineEvent::Translation { seg_id, lang, text, timing, epoch, .. } => {
                translations.entry(*seg_id).or_default().push((*lang, text.clone(), timing.published_ns, *epoch));
            }
            PipelineEvent::TranslationSkipped { seg_id, reason, .. } => skipped.entry(*seg_id).or_default().push(reason.clone()),
            PipelineEvent::Dropped { seg_id, stage, reason } => an.dropped.push(format!("seg {seg_id} @{stage}: {reason}")),
            PipelineEvent::Error { stage, message, .. } => an.errors.push(format!("{stage}: {message}")),
            PipelineEvent::State { detail: Some(d), .. } if d.starts_with("LANGUAGE_SWITCH") => an.switch_events.push(d.clone()),
            _ => {}
        }
    }
    offsets.sort();
    let t0 = offsets.get(offsets.len() / 2).copied().unwrap_or(0);
    an.t0_offset_ns = t0;
    let mut used = vec![false; transcripts.len()];
    let (mut edits, mut ref_len) = (0usize, 0usize);
    let mut chrf: HashMap<TargetLanguage, Vec<f64>> = HashMap::new();
    let (mut ls_en, mut le_en, mut ls_es, mut le_es) = (Vec::new(), Vec::new(), Vec::new(), Vec::new());
    for it in truth {
        let (Some(s), Some(e)) = (it.start_s, it.end_s) else { continue };
        let (s_ms, e_ms) = (((s * 1000.0) as i64 + offset_ms).max(0) as u64, ((e * 1000.0) as i64 + offset_ms).max(0) as u64);
        let mut u = UtteranceResult { id: it.id.clone(), reference_ja: it.ja.clone(), ..Default::default() };
        let mut first_pub: Option<(TargetLanguage, u64)> = None;
        let mut texts: Vec<String> = Vec::new();
        for (k, (id, ja, ts, te, _)) in transcripts.iter().enumerate() {
            let overlap = (*te).min(e_ms + 300).saturating_sub((*ts).max(s_ms.saturating_sub(300)));
            let own = te.saturating_sub(*ts).max(1);
            if !used[k] && overlap * 2 >= own {
                used[k] = true;
                u.segments.push(*id);
                u.asr.push_str(ja);
                if let Some(tr) = translations.get(id) {
                    for (l, t, p, _) in tr {
                        texts.push(t.clone());
                        u.lang = Some(l.code().into());
                        if first_pub.is_none_or(|(_, fp)| *p < fp) {
                            first_pub = Some((*l, *p));
                        }
                    }
                }
                if let Some(sk) = skipped.get(id) {
                    u.skipped.extend(sk.iter().cloned());
                }
            }
        }
        let (ed, n) = cer_counts(&u.asr, &it.ja);
        edits += ed;
        ref_len += n;
        u.cer = ed as f64 / n.max(1) as f64;
        u.translation = texts.join(" ");
        if let Some((l, p)) = first_pub {
            u.reference = if l == TargetLanguage::En { it.en.clone() } else { it.es.clone() };
            u.chrf = chrf_fn(&u.translation, &u.reference);
            chrf.entry(l).or_default().push(u.chrf);
            let st = t0 + (s_ms as i64) * 1_000_000;
            let en = t0 + (e_ms as i64) * 1_000_000;
            let a_ms = (p as i64 - st) as f64 / 1e6;
            let b_ms = (p as i64 - en) as f64 / 1e6;
            u.start_to_subtitle_ms = Some(a_ms);
            u.end_to_subtitle_ms = Some(b_ms);
            if l == TargetLanguage::En {
                ls_en.push(a_ms);
                le_en.push(b_ms);
            } else {
                ls_es.push(a_ms);
                le_es.push(b_ms);
            }
        }
        an.utterances.push(u);
    }
    for (k, (id, ja, ..)) in transcripts.iter().enumerate() {
        if !used[k] {
            an.unmatched_segments.push((*id, ja.clone()));
        }
    }
    // stale-language check: any translation whose epoch is older than the latest
    // epoch seen at an earlier publication is a stale publish.
    let mut pubs: Vec<(u64, u64)> = translations.values().flatten().map(|(_, _, p, e)| (*p, *e)).collect();
    pubs.sort();
    let mut max_epoch = 0;
    for (_, e) in pubs {
        if e < max_epoch {
            an.stale_language_after_switch += 1;
        }
        max_epoch = max_epoch.max(e);
    }
    an.corpus_cer = edits as f64 / ref_len.max(1) as f64;
    let mean = |v: Option<&Vec<f64>>| v.filter(|v| !v.is_empty()).map(|v| v.iter().sum::<f64>() / v.len() as f64);
    an.mean_chrf_en = mean(chrf.get(&TargetLanguage::En));
    an.mean_chrf_es = mean(chrf.get(&TargetLanguage::Es));
    an.latency_start_en = summarize(ls_en);
    an.latency_end_en = summarize(le_en);
    an.latency_start_es = summarize(ls_es);
    an.latency_end_es = summarize(le_es);
    an
}

fn chrf_fn(h: &str, r: &str) -> f64 {
    chrf(h, r)
}

fn fmt_s(s: &Summary) -> String {
    if s.n == 0 { "n/a".into() } else { format!("n={} median {:.0} / p95 {:.0} / max {:.0}", s.n, s.median, s.p95, s.max) }
}

pub fn pipeline_markdown(src: &str, an: &Analysis, st: &PipelineStats, stop: &StopReport) -> String {
    let mut md = format!("# Pipeline run: `{src}`\n\n");
    md.push_str(&format!(
        "- corpus CER (ASR vs truth): {:.3}\n- mean chrF en: {:?}, es: {:?}\n- speech start -> subtitle (en): {}\n- speech end -> subtitle (en): {}\n- speech start -> subtitle (es): {}\n- speech end -> subtitle (es): {}\n",
        an.corpus_cer,
        an.mean_chrf_en.map(|v| (v * 10.0).round() / 10.0),
        an.mean_chrf_es.map(|v| (v * 10.0).round() / 10.0),
        fmt_s(&an.latency_start_en),
        fmt_s(&an.latency_end_en),
        fmt_s(&an.latency_start_es),
        fmt_s(&an.latency_end_es)
    ));
    md.push_str(&format!(
        "- ASR ms: {}\n- MT ms en: {} | es: {}\n- MT queue wait en: {} | es: {}\n",
        fmt_s(&st.latency.asr_ms),
        fmt_s(&st.latency.en.mt_ms),
        fmt_s(&st.latency.es.mt_ms),
        fmt_s(&st.latency.en.mt_queue_ms),
        fmt_s(&st.latency.es.mt_queue_ms)
    ));
    md.push_str(&format!(
        "- segments {} (forced cuts {}, blips {}), filtered {}, dropped asr-queue {} / asr-stale {} / mt-queue {} / mt-stale {}\n- translation failures {}, worker restarts {}, coordinator {:?}\n- stale-language publications after switch: {}\n- switches: {:?}\n- stop: {:.0} ms (joined {})\n- capture: {:?}\n\n",
        st.segmenter.segments,
        st.segmenter.forced_cuts,
        st.segmenter.discarded_blips,
        st.filtered,
        st.dropped_asr_queue,
        st.dropped_asr_stale,
        st.dropped_mt_queue,
        st.skipped_mt_stale,
        st.translation_failures,
        st.translator_restarts,
        st.coordinator,
        an.stale_language_after_switch,
        an.switch_events,
        stop.stop_ms,
        stop.threads_joined,
        st.capture
    ));
    md.push_str("| id | reference ja | ASR | CER | lang | reference | subtitle | chrF | start→sub ms | end→sub ms |\n|---|---|---|---|---|---|---|---|---|---|\n");
    for u in &an.utterances {
        let skipped = u.skipped.join("; ");
        md.push_str(&format!(
            "| {} | {} | {} | {:.2} | {} | {} | {} | {:.0} | {} | {} |\n",
            u.id,
            crate::md_escape(&u.reference_ja),
            crate::md_escape(&u.asr),
            u.cer,
            u.lang.as_deref().unwrap_or("-"),
            crate::md_escape(&u.reference),
            crate::md_escape(if u.translation.is_empty() { &skipped } else { &u.translation }),
            u.chrf,
            u.start_to_subtitle_ms.map(|v| format!("{v:.0}")).unwrap_or("-".into()),
            u.end_to_subtitle_ms.map(|v| format!("{v:.0}")).unwrap_or("-".into()),
        ));
    }
    if !an.unmatched_segments.is_empty() {
        md.push_str(&format!("\nunmatched segments (possible hallucinations / split lines): {:?}\n", an.unmatched_segments));
    }
    if !an.errors.is_empty() {
        md.push_str(&format!("\nerrors: {:?}\n", an.errors));
    }
    if !an.dropped.is_empty() {
        md.push_str(&format!("\ndropped: {:?}\n", an.dropped));
    }
    md
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chrf_bounds() {
        assert!((chrf("I won't go.", "I won't go.") - 100.0).abs() < 1e-9);
        assert!(chrf("completely different", "nothing alike here") < 30.0);
        assert_eq!(chrf("", "x"), 0.0);
    }

    #[test]
    fn cer_ignores_punctuation() {
        assert_eq!(cer_counts("全然怖くないし", "全然怖くないし。"), (0, 7));
        assert_eq!(cer_counts("全然怖いし", "全然怖くないし").0, 2);
    }
}
