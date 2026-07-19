//! Track-aware topic selection.

use std::collections::BTreeSet;

use crate::{
    interest::{interest_score, is_eligible},
    model::{EnglishTrack, LearningSkill, TopicCandidate},
};

/// Runtime-configured scope for one OPIc practice activity.
///
/// The crate deliberately does not hard-code claims about the current exam.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OpicScope {
    pub activity_id: String,
    pub allowed_topic_tags: BTreeSet<String>,
}

impl OpicScope {
    pub fn new(
        activity_id: impl Into<String>,
        allowed_topic_tags: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        Self {
            activity_id: activity_id.into(),
            allowed_topic_tags: allowed_topic_tags.into_iter().map(Into::into).collect(),
        }
    }

    fn allows(&self, candidate: &TopicCandidate) -> bool {
        candidate
            .opic_scope_tags
            .iter()
            .any(|tag| self.allowed_topic_tags.contains(tag))
    }
}

#[derive(Clone, Debug)]
pub struct StudyRequest<'a> {
    pub track: EnglishTrack,
    pub skill: LearningSkill,
    pub opic_scope: Option<&'a OpicScope>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TopicSelection {
    pub topic_id: String,
    pub topic_title: String,
    pub reason: SelectionReason,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SelectionReason {
    IntegratedInterest,
    OpicScopeThenInterest,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanError {
    MissingOpicScope,
    NoEligibleTopic,
}

/// Selects a topic without allowing interest to override track policy.
pub fn select_topic(
    request: &StudyRequest<'_>,
    candidates: &[TopicCandidate],
) -> Result<TopicSelection, PlanError> {
    let opic_scope = match request.track {
        EnglishTrack::Opic => Some(request.opic_scope.ok_or(PlanError::MissingOpicScope)?),
        EnglishTrack::Integrated => None,
    };

    let candidate = candidates
        .iter()
        .filter(|candidate| is_eligible(candidate))
        .filter(|candidate| {
            opic_scope.is_none_or(|configured_scope| configured_scope.allows(candidate))
        })
        .max_by(|left, right| {
            interest_score(left)
                .cmp(&interest_score(right))
                // Stable deterministic tie-breaker; smaller ID wins.
                .then_with(|| right.id.cmp(&left.id))
        })
        .ok_or(PlanError::NoEligibleTopic)?;

    Ok(TopicSelection {
        topic_id: candidate.id.clone(),
        topic_title: candidate.title.clone(),
        reason: match request.track {
            EnglishTrack::Opic => SelectionReason::OpicScopeThenInterest,
            EnglishTrack::Integrated => SelectionReason::IntegratedInterest,
        },
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        interest::InterestSignal,
        model::{EnglishTrack, InterestGroup, LearningSkill, TopicCandidate},
    };

    fn interested_robotics() -> TopicCandidate {
        TopicCandidate::study(
            "robotics",
            "Building a robot controller",
            InterestGroup::PrimaryComputingSystems,
        )
        .with_opic_scope_tags(["hobby"])
        .with_signal(InterestSignal::FollowUpQuestion)
    }

    #[test]
    fn integrated_english_uses_high_interest_systems_topic() {
        let topics = [
            TopicCandidate::study("generic", "Generic topic", InterestGroup::Other),
            interested_robotics(),
        ];
        let request = StudyRequest {
            track: EnglishTrack::Integrated,
            skill: LearningSkill::Reading,
            opic_scope: None,
        };

        let selected = select_topic(&request, &topics).expect("a topic should be selected");
        assert_eq!(selected.topic_id, "robotics");
        assert_eq!(selected.reason, SelectionReason::IntegratedInterest);
    }

    #[test]
    fn opic_rejects_interesting_but_out_of_scope_topic() {
        let out_of_scope = TopicCandidate::study(
            "rtl-paper",
            "An RTL verification paper",
            InterestGroup::PrimaryComputingSystems,
        )
        .with_opic_scope_tags(["paper-analysis"])
        .with_signal(InterestSignal::PinnedInterest);
        let in_scope = TopicCandidate::study(
            "weekend-project",
            "A weekend robot project",
            InterestGroup::PrimaryComputingSystems,
        )
        .with_opic_scope_tags(["hobby"]);
        let scope = OpicScope::new("configured-hobby-practice", ["hobby"]);
        let request = StudyRequest {
            track: EnglishTrack::Opic,
            skill: LearningSkill::Speaking,
            opic_scope: Some(&scope),
        };

        let selected = select_topic(&request, &[out_of_scope, in_scope])
            .expect("the in-scope example should be selected");
        assert_eq!(selected.topic_id, "weekend-project");
        assert_eq!(selected.reason, SelectionReason::OpicScopeThenInterest);
    }

    #[test]
    fn opic_requires_an_explicit_runtime_scope() {
        let request = StudyRequest {
            track: EnglishTrack::Opic,
            skill: LearningSkill::Speaking,
            opic_scope: None,
        };

        assert_eq!(
            select_topic(&request, &[interested_robotics()]),
            Err(PlanError::MissingOpicScope)
        );
    }
}
