//! Discord channel definitions.
//!
//! `order_ref` is documentation/configuration metadata. It must never be
//! rendered as part of `display_name`; the user controls Discord ordering.

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum EnglishCategory {
    Opic,
    Integrated,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ChannelDefinition {
    pub category: EnglishCategory,
    pub order_ref: u16,
    pub id: &'static str,
    pub display_name: &'static str,
}

const OPIC_CHANNELS: &[ChannelDefinition] = &[
    channel(EnglishCategory::Opic, 0, "guide", "안내"),
    channel(EnglishCategory::Opic, 10, "vocabulary", "단어학습"),
    channel(EnglishCategory::Opic, 11, "expressions", "표현학습"),
    channel(EnglishCategory::Opic, 12, "pronunciation", "발음과억양"),
    channel(EnglishCategory::Opic, 20, "daily-practice", "일일학습"),
    channel(EnglishCategory::Opic, 30, "mock-exam", "모의고사"),
    channel(EnglishCategory::Opic, 40, "error-review", "오답복습"),
    channel(EnglishCategory::Opic, 80, "progress", "학습현황"),
    channel(EnglishCategory::Opic, 90, "newsletter", "뉴스레터"),
];

const INTEGRATED_CHANNELS: &[ChannelDefinition] = &[
    channel(EnglishCategory::Integrated, 0, "guide", "안내"),
    channel(EnglishCategory::Integrated, 10, "vocabulary", "어휘"),
    channel(
        EnglishCategory::Integrated,
        11,
        "grammar-and-syntax",
        "문법과구문",
    ),
    channel(
        EnglishCategory::Integrated,
        12,
        "pronunciation-rhythm-intonation",
        "발음리듬억양",
    ),
    channel(EnglishCategory::Integrated, 20, "reading", "읽기"),
    channel(
        EnglishCategory::Integrated,
        21,
        "paper-and-professional-reading",
        "논문과전문읽기",
    ),
    channel(EnglishCategory::Integrated, 30, "writing", "쓰기"),
    channel(
        EnglishCategory::Integrated,
        31,
        "purposeful-writing",
        "목적별쓰기",
    ),
    channel(EnglishCategory::Integrated, 40, "listening", "듣기"),
    channel(
        EnglishCategory::Integrated,
        41,
        "conversation-presentation-discussion",
        "대화발표토론",
    ),
    channel(
        EnglishCategory::Integrated,
        50,
        "integrated-practice",
        "통합실습",
    ),
    channel(
        EnglishCategory::Integrated,
        60,
        "source-library",
        "통합자료실",
    ),
    channel(EnglishCategory::Integrated, 80, "progress", "학습현황"),
    channel(
        EnglishCategory::Integrated,
        90,
        "english-reading-material",
        "영어읽을거리",
    ),
];

const fn channel(
    category: EnglishCategory,
    order_ref: u16,
    id: &'static str,
    display_name: &'static str,
) -> ChannelDefinition {
    ChannelDefinition {
        category,
        order_ref,
        id,
        display_name,
    }
}

pub fn english_channels() -> impl Iterator<Item = &'static ChannelDefinition> {
    OPIC_CHANNELS.iter().chain(INTEGRATED_CHANNELS)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn discord_display_names_do_not_contain_numeric_order_prefixes() {
        for channel in english_channels() {
            assert!(
                !channel
                    .display_name
                    .chars()
                    .next()
                    .is_some_and(|character| character.is_ascii_digit()),
                "{} must not expose order_ref in Discord",
                channel.display_name
            );
        }
    }

    #[test]
    fn channel_ids_are_unique_within_each_category() {
        let mut seen = HashSet::new();
        for channel in english_channels() {
            assert!(seen.insert((channel.category, channel.id)));
        }
    }
}
