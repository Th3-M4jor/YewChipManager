use serde::Deserialize;
use std::cmp::{Ord, Ordering};
#[derive(Deserialize, Clone, Copy)]
pub(crate) enum ChipType {
    Standard,
    Mega,
    Giga,
    Dark,
    Support,
}

impl std::default::Default for ChipType {
    fn default() -> Self {
        ChipType::Standard
    }
}

impl ChipType {
    #[inline]
    fn to_num(&self) -> u32 {
        match self {
            ChipType::Standard | ChipType::Support => 0,
            ChipType::Mega => 1,
            ChipType::Giga => 2,
            ChipType::Dark => 3,
        }
    }

    #[inline]
    pub(crate) fn to_css_class(&self) -> &'static str {
        match self {
            ChipType::Standard => {"Chip"}
            ChipType::Mega => {"Mega"}
            ChipType::Giga => {"Giga"}
            ChipType::Dark => {"unknownChip"}
            ChipType::Support => {"SupportChip"}
        }
    }

    #[inline]
    pub(crate) fn to_background_css_class(&self) -> &'static str {
        match self {
            ChipType::Standard => {"chipDescBackgroundStd"}
            ChipType::Mega => {"chipDescBackgroundMega"}
            ChipType::Giga => {"chipDescBackgroundGiga"}
            ChipType::Dark => {"chipDescBackgroundDark"}
            ChipType::Support => {"chipDescBackgroundSupprt"}
        }
    }

}

impl PartialOrd for ChipType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ChipType {
    fn cmp(&self, other: &Self) -> Ordering {
        let first = self.to_num();
        let second = other.to_num();
        first.cmp(&second)
    }
}

impl PartialEq for ChipType {
    fn eq(&self, other: &Self) -> bool {
        self.to_num() == other.to_num()
    }
}

impl Eq for ChipType {}