//! Latency bookkeeping. Each definition is measured separately and never mixed:
//! speech start -> subtitle, speech end -> subtitle, ASR time, MT time, queue
//! waits and UI update. Only the most recent samples are kept (bounded memory).

use std::collections::{HashMap, VecDeque};

use serde::{Deserialize, Serialize};

use crate::lang::TargetLanguage;

const MAX_SAMPLES: usize = 2_000;

#[derive(Clone, Debug, Default)]
pub struct Series {
    values: VecDeque<f64>,
}

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct Summary {
    pub n: usize,
    pub median: f64,
    pub p95: f64,
    pub max: f64,
    pub mean: f64,
}

impl Series {
    pub fn push(&mut self, v: f64) {
        if !v.is_finite() {
            return;
        }
        if self.values.len() == MAX_SAMPLES {
            self.values.pop_front();
        }
        self.values.push_back(v);
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    pub fn summary(&self) -> Summary {
        summarize(self.values.iter().copied().collect())
    }
}

pub fn summarize(mut v: Vec<f64>) -> Summary {
    if v.is_empty() {
        return Summary::default();
    }
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let n = v.len();
    let pct = |p: f64| {
        let rank = (p * (n - 1) as f64).round() as usize;
        v[rank.min(n - 1)]
    };
    Summary { n, median: pct(0.5), p95: pct(0.95), max: v[n - 1], mean: v.iter().sum::<f64>() / n as f64 }
}

/// Timestamps (pipeline-clock ns) collected for one segment as it moves through.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct SegmentTiming {
    pub seg_id: u64,
    pub speech_start_ns: u64,
    pub speech_end_ns: u64,
    pub segment_ready_ns: u64,
    pub asr_start_ns: u64,
    pub asr_end_ns: u64,
    pub transcript_published_ns: u64,
    pub mt_start_ns: u64,
    pub mt_end_ns: u64,
    pub published_ns: u64,
}

fn ms(a: u64, b: u64) -> Option<f64> {
    if a == 0 || b == 0 || b < a { None } else { Some((b - a) as f64 / 1e6) }
}

#[derive(Clone, Debug, Default)]
pub struct LanguageLatency {
    pub speech_start_to_subtitle: Series,
    pub speech_end_to_subtitle: Series,
    pub mt_ms: Series,
    pub mt_queue_ms: Series,
    pub ui_ms: Series,
}

#[derive(Clone, Debug, Default)]
pub struct LatencyBook {
    pub asr_ms: Series,
    pub asr_queue_ms: Series,
    pub speech_end_to_transcript: Series,
    pub per_lang: HashMap<TargetLanguage, LanguageLatency>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct LanguageLatencyReport {
    pub speech_start_to_subtitle_ms: Summary,
    pub speech_end_to_subtitle_ms: Summary,
    pub mt_ms: Summary,
    pub mt_queue_ms: Summary,
    pub ui_ms: Summary,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct LatencyReport {
    pub asr_ms: Summary,
    pub asr_queue_ms: Summary,
    pub speech_end_to_transcript_ms: Summary,
    pub en: LanguageLatencyReport,
    pub es: LanguageLatencyReport,
}

impl LatencyBook {
    pub fn record_transcript(&mut self, t: &SegmentTiming) {
        if let Some(v) = ms(t.asr_start_ns, t.asr_end_ns) {
            self.asr_ms.push(v);
        }
        if let Some(v) = ms(t.segment_ready_ns, t.asr_start_ns) {
            self.asr_queue_ms.push(v);
        }
        if let Some(v) = ms(t.speech_end_ns, t.transcript_published_ns) {
            self.speech_end_to_transcript.push(v);
        }
    }

    pub fn record_translation(&mut self, lang: TargetLanguage, t: &SegmentTiming) {
        let l = self.per_lang.entry(lang).or_default();
        if let Some(v) = ms(t.speech_start_ns, t.published_ns) {
            l.speech_start_to_subtitle.push(v);
        }
        if let Some(v) = ms(t.speech_end_ns, t.published_ns) {
            l.speech_end_to_subtitle.push(v);
        }
        if let Some(v) = ms(t.mt_start_ns, t.mt_end_ns) {
            l.mt_ms.push(v);
        }
        if let Some(v) = ms(t.asr_end_ns, t.mt_start_ns) {
            l.mt_queue_ms.push(v);
        }
    }

    pub fn record_ui(&mut self, lang: TargetLanguage, ms: f64) {
        self.per_lang.entry(lang).or_default().ui_ms.push(ms);
    }

    pub fn report(&self) -> LatencyReport {
        let lang = |l: TargetLanguage| {
            self.per_lang
                .get(&l)
                .map(|x| LanguageLatencyReport {
                    speech_start_to_subtitle_ms: x.speech_start_to_subtitle.summary(),
                    speech_end_to_subtitle_ms: x.speech_end_to_subtitle.summary(),
                    mt_ms: x.mt_ms.summary(),
                    mt_queue_ms: x.mt_queue_ms.summary(),
                    ui_ms: x.ui_ms.summary(),
                })
                .unwrap_or_default()
        };
        LatencyReport {
            asr_ms: self.asr_ms.summary(),
            asr_queue_ms: self.asr_queue_ms.summary(),
            speech_end_to_transcript_ms: self.speech_end_to_transcript.summary(),
            en: lang(TargetLanguage::En),
            es: lang(TargetLanguage::Es),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn summary_percentiles() {
        let s = summarize((1..=100).map(|x| x as f64).collect());
        assert_eq!(s.n, 100);
        assert_eq!(s.max, 100.0);
        assert!((s.median - 50.0).abs() <= 1.0);
        assert!((s.p95 - 95.0).abs() <= 1.0);
    }

    #[test]
    fn series_is_bounded() {
        let mut s = Series::default();
        for i in 0..(MAX_SAMPLES * 3) {
            s.push(i as f64);
        }
        assert_eq!(s.len(), MAX_SAMPLES);
    }
}
