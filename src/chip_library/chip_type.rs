use serde::Deserialize;
use std::cmp::{Ord, Ordering};
#[derive(Deserialize, Clone, Copy)]
pub enum ChipType {
    Standard,
    Mega,
    Giga,
    Dark,
    Support,
}

impl std::fmt::Display for ChipType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChipType::Standard => write!(f, ""),
            ChipType::Mega => write!(f, "Mega"),
            ChipType::Giga => write!(f, "Giga"),
            ChipType::Dark => write!(f, "Dark"),
            ChipType::Support => write!(f, "Support"),
        }
    }
}

impl std::default::Default for ChipType {
    fn default() -> Self {
        ChipType::Standard
    }
}

impl ChipType {
    #[inline]
    fn to_num(&self) -> u32 {
        return match self {
            ChipType::Standard | ChipType::Support => 0,
            ChipType::Mega => 1,
            ChipType::Giga => 2,
            ChipType::Dark => 3,
        }
    }

    #[inline]
    pub fn to_css_class(&self) -> &'static str {
        match self {
            ChipType::Standard => {"Chip"}
            ChipType::Mega => {"Mega"}
            ChipType::Giga => {"Giga"}
            ChipType::Dark => {"unknownChip"}
            ChipType::Support => {"SupportChip"}
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