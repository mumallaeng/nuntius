//! Deterministic interpretation of cross-space interest signals.

use crate::model::{EnglishReuse, InterestGroup, PrivacyClass, TopicCandidate};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum InterestSignal {
    FollowUpQuestion,
    LearningThreadCreated,
    SavedSource,
    ExplicitEnglishReuse,
    PinnedInterest,
    PositiveReaction,
    ReadLater,
    PassiveDelivery,
    LowerInterest,
    Excluded,
}

/// Returns whether a topic may cross into an English-learning session.
pub fn is_eligible(candidate: &TopicCandidate) -> bool {
    if candidate.english_reuse == EnglishReuse::Denied
        || candidate.signals.contains(&InterestSignal::Excluded)
    {
        return false;
    }

    candidate.privacy != PrivacyClass::PersonalSensitive
        || candidate.english_reuse == EnglishReuse::Allowed
        || candidate
            .signals
            .contains(&InterestSignal::ExplicitEnglishReuse)
}

/// Computes a stable ranking score. The score is an implementation detail,
/// not a probability or a fixed publishing quota.
pub fn interest_score(candidate: &TopicCandidate) -> i32 {
    if !is_eligible(candidate) {
        return i32::MIN;
    }

    let baseline = match candidate.group {
        // These two groups intentionally start in the same priority band.
        InterestGroup::PrimaryComputingSystems | InterestGroup::GeneralStem => 20,
        InterestGroup::Literature => 10,
        InterestGroup::Other => 0,
    };

    candidate
        .signals
        .iter()
        .fold(baseline, |score, signal| score + signal_score(*signal))
}

const fn signal_score(signal: InterestSignal) -> i32 {
    match signal {
        InterestSignal::FollowUpQuestion
        | InterestSignal::ExplicitEnglishReuse
        | InterestSignal::PinnedInterest => 50,
        InterestSignal::LearningThreadCreated => 40,
        InterestSignal::SavedSource => 35,
        InterestSignal::PositiveReaction => 15,
        InterestSignal::ReadLater => 10,
        InterestSignal::PassiveDelivery => 0,
        InterestSignal::LowerInterest => -20,
        InterestSignal::Excluded => -10_000,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{InterestGroup, TopicCandidate};

    #[test]
    fn primary_systems_and_general_stem_have_equal_initial_weight() {
        let systems = TopicCandidate::study(
            "robot-control",
            "Robot control",
            InterestGroup::PrimaryComputingSystems,
        );
        let dinosaurs = TopicCandidate::study(
            "feathered-dinosaurs",
            "Feathered dinosaurs",
            InterestGroup::GeneralStem,
        );

        assert_eq!(interest_score(&systems), interest_score(&dinosaurs));
    }

    #[test]
    fn passive_delivery_does_not_create_extra_interest() {
        let baseline = TopicCandidate::study("rtl", "RTL", InterestGroup::Other);
        let delivered = baseline
            .clone()
            .with_signal(InterestSignal::PassiveDelivery);

        assert_eq!(interest_score(&baseline), interest_score(&delivered));
    }

    #[test]
    fn sensitive_topic_requires_explicit_reuse() {
        let mut sensitive =
            TopicCandidate::study("private-account", "Private account", InterestGroup::Other);
        sensitive.privacy = PrivacyClass::PersonalSensitive;
        assert!(!is_eligible(&sensitive));

        sensitive.english_reuse = EnglishReuse::Allowed;
        assert!(is_eligible(&sensitive));
    }
}
