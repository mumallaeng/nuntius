//! Shared English-learning domain types.

/// An English-learning track with its own personalization boundary.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EnglishTrack {
    /// Goal-specific OPIc practice. Configured OPIc scope remains authoritative.
    Opic,
    /// Long-term English capability across skills and usage contexts.
    Integrated,
}

/// A capability-oriented learning activity.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LearningSkill {
    Vocabulary,
    GrammarAndSyntax,
    Pronunciation,
    Reading,
    Writing,
    Listening,
    Speaking,
}

/// Where the learner expects to use the language.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UsageContext {
    DailyLife,
    SocialInteraction,
    TravelAndService,
    WorkplaceAndBusiness,
    AcademicCoursework,
    ResearchAndPublication,
    TechnicalAndProfessional,
    PublicLifeAdministration,
    MediaAndCulture,
}

/// Broad topic groups used to initialize, not freeze, personalization.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum InterestGroup {
    /// Robotics and the hardware-firmware-software continuum.
    PrimaryComputingSystems,
    /// Mathematics, science, paleontology, and other engineering.
    GeneralStem,
    Literature,
    Other,
}

/// Privacy controls are evaluated before topic relevance.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PrivacyClass {
    Public,
    Study,
    PersonalSensitive,
}

/// Item-level permission to reuse a source for English learning.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EnglishReuse {
    Default,
    Allowed,
    Denied,
}

/// A candidate topic with source-preserving metadata.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TopicCandidate {
    pub id: String,
    pub title: String,
    pub group: InterestGroup,
    pub privacy: PrivacyClass,
    pub english_reuse: EnglishReuse,
    /// Configured OPIc scope tags. They are data, not hard-coded exam claims.
    pub opic_scope_tags: Vec<String>,
    pub signals: Vec<crate::interest::InterestSignal>,
}

impl TopicCandidate {
    /// Creates a non-sensitive topic that may be used by integrated English.
    pub fn study(id: impl Into<String>, title: impl Into<String>, group: InterestGroup) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            group,
            privacy: PrivacyClass::Study,
            english_reuse: EnglishReuse::Default,
            opic_scope_tags: Vec::new(),
            signals: Vec::new(),
        }
    }

    pub fn with_opic_scope_tags(
        mut self,
        tags: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        self.opic_scope_tags = tags.into_iter().map(Into::into).collect();
        self
    }

    pub fn with_signal(mut self, signal: crate::interest::InterestSignal) -> Self {
        self.signals.push(signal);
        self
    }
}
