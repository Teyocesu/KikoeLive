//! Final gate before anything is shown. It enforces the publication invariants:
//! * subtitles appear in segment order (never an older line after a newer one),
//! * a translation started before a language switch is never shown after it,
//! * a translation is never shown under the wrong language label,
//! * the same line is not published twice.

use serde::{Deserialize, Serialize};

use crate::lang::{TargetLanguage, guess_latin_language};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RejectReason {
    StaleEpoch,
    WrongLanguage,
    OutOfOrder,
    Duplicate,
}

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize)]
pub struct CoordinatorStats {
    pub transcripts: u64,
    pub translations: u64,
    pub stale_epoch: u64,
    pub wrong_language: u64,
    pub out_of_order: u64,
    pub duplicate: u64,
}

#[derive(Default)]
pub struct SubtitleCoordinator {
    last_transcript_seg: u64,
    last_translation_seg: u64,
    stats: CoordinatorStats,
}

impl SubtitleCoordinator {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn stats(&self) -> CoordinatorStats {
        self.stats
    }

    pub fn offer_transcript(&mut self, seg_id: u64) -> Result<(), RejectReason> {
        if seg_id <= self.last_transcript_seg {
            self.stats.out_of_order += 1;
            return Err(if seg_id == self.last_transcript_seg { RejectReason::Duplicate } else { RejectReason::OutOfOrder });
        }
        self.last_transcript_seg = seg_id;
        self.stats.transcripts += 1;
        Ok(())
    }

    /// `job` is the (language, epoch) the translation was produced under,
    /// `current` the selection at publication time.
    pub fn offer_translation(
        &mut self,
        seg_id: u64,
        job: (TargetLanguage, u64),
        current: (TargetLanguage, u64),
        text: &str,
    ) -> Result<(), RejectReason> {
        if job.1 != current.1 || job.0 != current.0 {
            self.stats.stale_epoch += 1;
            return Err(RejectReason::StaleEpoch);
        }
        if let Some(guess) = guess_latin_language(text) {
            if guess != job.0 {
                self.stats.wrong_language += 1;
                return Err(RejectReason::WrongLanguage);
            }
        }
        if seg_id < self.last_translation_seg {
            self.stats.out_of_order += 1;
            return Err(RejectReason::OutOfOrder);
        }
        if seg_id == self.last_translation_seg {
            self.stats.duplicate += 1;
            return Err(RejectReason::Duplicate);
        }
        self.last_translation_seg = seg_id;
        self.stats.translations += 1;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use TargetLanguage::*;

    #[test]
    fn stale_translation_after_switch_is_rejected() {
        let mut c = SubtitleCoordinator::new();
        // job started in English under epoch 1, user switched to Spanish (epoch 2)
        assert_eq!(c.offer_translation(1, (En, 1), (Es, 2), "I'm not going."), Err(RejectReason::StaleEpoch));
        assert_eq!(c.offer_translation(1, (Es, 2), (Es, 2), "No voy a ir."), Ok(()));
    }

    #[test]
    fn wrong_language_output_is_rejected() {
        let mut c = SubtitleCoordinator::new();
        assert_eq!(
            c.offer_translation(3, (Es, 2), (Es, 2), "I don't know what you mean."),
            Err(RejectReason::WrongLanguage)
        );
        assert_eq!(c.stats().wrong_language, 1);
    }

    #[test]
    fn ordering_and_duplicates_are_enforced() {
        let mut c = SubtitleCoordinator::new();
        assert!(c.offer_translation(5, (En, 1), (En, 1), "Five.").is_ok());
        assert_eq!(c.offer_translation(4, (En, 1), (En, 1), "Four."), Err(RejectReason::OutOfOrder));
        assert_eq!(c.offer_translation(5, (En, 1), (En, 1), "Five."), Err(RejectReason::Duplicate));
        assert!(c.offer_translation(6, (En, 1), (En, 1), "Six.").is_ok());
        assert!(c.offer_transcript(6).is_ok());
        assert!(c.offer_transcript(6).is_err());
        assert!(c.offer_transcript(2).is_err());
    }
}
