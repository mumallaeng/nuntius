//! Lingua local-first pronunciation practice policy.

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PronunciationStage {
    SegmentalSounds,
    SyllablesAndWordStress,
    SentenceStressAndRhythm,
    LinkingReductionAndWeakForms,
    Intonation,
    ContextualIntelligibility,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AudioRetention {
    SessionOnly,
    ExplicitlySavedExemplar,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EvidenceKind {
    Sound,
    Stress,
    Rhythm,
    Linking,
    Intonation,
    Intelligibility,
    /// ASR is useful for delivered words, but it is not a pronunciation score.
    AuxiliaryAsrTranscript,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PronunciationPolicy {
    pub stages: Vec<PronunciationStage>,
    pub feedback_limit_per_attempt: usize,
    pub local_first: bool,
    pub default_audio_retention: AudioRetention,
}

impl Default for PronunciationPolicy {
    fn default() -> Self {
        Self {
            stages: vec![
                PronunciationStage::SegmentalSounds,
                PronunciationStage::SyllablesAndWordStress,
                PronunciationStage::SentenceStressAndRhythm,
                PronunciationStage::LinkingReductionAndWeakForms,
                PronunciationStage::Intonation,
                PronunciationStage::ContextualIntelligibility,
            ],
            feedback_limit_per_attempt: 2,
            local_first: true,
            default_audio_retention: AudioRetention::SessionOnly,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PronunciationObservation {
    pub kind: EvidenceKind,
    /// Relative impact on intelligibility. It is not a universal score.
    pub impact: u8,
    pub guidance: String,
}

/// Keeps feedback focused on the highest-impact corrections.
pub fn prioritize_feedback(
    policy: &PronunciationPolicy,
    observations: &[PronunciationObservation],
) -> Vec<PronunciationObservation> {
    let mut ranked: Vec<_> = observations
        .iter()
        // ASR transcript evidence is displayed separately from corrections.
        .filter(|observation| observation.kind != EvidenceKind::AuxiliaryAsrTranscript)
        .cloned()
        .collect();

    ranked.sort_by(|left, right| right.impact.cmp(&left.impact));
    ranked.truncate(policy.feedback_limit_per_attempt);
    ranked
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_policy_is_local_first_and_session_scoped() {
        let policy = PronunciationPolicy::default();

        assert!(policy.local_first);
        assert_eq!(policy.feedback_limit_per_attempt, 2);
        assert_eq!(policy.default_audio_retention, AudioRetention::SessionOnly);
    }

    #[test]
    fn feedback_is_limited_and_asr_is_not_a_correction() {
        let policy = PronunciationPolicy::default();
        let observations = [
            PronunciationObservation {
                kind: EvidenceKind::AuxiliaryAsrTranscript,
                impact: 100,
                guidance: "recognized transcript".into(),
            },
            PronunciationObservation {
                kind: EvidenceKind::Rhythm,
                impact: 80,
                guidance: "reduce unstressed syllables".into(),
            },
            PronunciationObservation {
                kind: EvidenceKind::Intelligibility,
                impact: 90,
                guidance: "make the keyword clearer".into(),
            },
            PronunciationObservation {
                kind: EvidenceKind::Intonation,
                impact: 20,
                guidance: "try a smaller final rise".into(),
            },
        ];

        let selected = prioritize_feedback(&policy, &observations);
        assert_eq!(selected.len(), 2);
        assert_eq!(selected[0].kind, EvidenceKind::Intelligibility);
        assert_eq!(selected[1].kind, EvidenceKind::Rhythm);
    }
}
