use serde::Deserialize;
use std::cmp::{Ord, Ordering};
#[derive(Deserialize, Clone, Copy)]
pub(crate) enum ChipClass {
    Standard,
    Mega,
    Giga,
    Dark,
    Support,
}

impl std::default::Default for ChipClass {
    fn default() -> Self {
        ChipClass::Standard
    }
}

impl ChipClass {

    fn to_num(&self) -> u32 {
        match self {
            ChipClass::Standard | ChipClass::Support => 0,
            ChipClass::Mega => 1,
            ChipClass::Giga => 2,
            ChipClass::Dark => 3,
        }
    }

    pub(crate) fn to_css_class(&self) -> &'static str {
        match self {
            ChipClass::Standard => {"Chip"}
            ChipClass::Mega => {"Mega"}
            ChipClass::Giga => {"Giga"}
            ChipClass::Dark => {"unknownChip"}
            ChipClass::Support => {"SupportChip"}
        }
    }

    pub(crate) fn to_background_css_class(&self) -> &'static str {
        match self {
            ChipClass::Standard => {"chipDescBackgroundStd"}
            ChipClass::Mega => {"chipDescBackgroundMega"}
            ChipClass::Giga => {"chipDescBackgroundGiga"}
            ChipClass::Dark => {"chipDescBackgroundDark"}
            ChipClass::Support => {"chipDescBackgroundSupprt"}
        }
    }

    pub(crate) fn max_in_folder(&self) -> u8 {
        match self {
            ChipClass::Standard | ChipClass::Support => 3,
            ChipClass::Mega | ChipClass::Giga | ChipClass::Dark => 1,
        }
    }

}

impl PartialOrd for ChipClass {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ChipClass {
    fn cmp(&self, other: &Self) -> Ordering {
        let first = self.to_num();
        let second = other.to_num();
        first.cmp(&second)
    }
}

impl PartialEq for ChipClass {
    fn eq(&self, other: &Self) -> bool {
        self.to_num() == other.to_num()
    }
}

impl Eq for ChipClass {}

#[derive(Deserialize, Clone, Copy)]
pub(crate) enum ChipType {
    Burst,
    Construct,
    Melee,
    Projectile,
    Wave,
    Recovery,
    Summon,
    Support,
    Trap,
}

impl ChipType {
    pub(crate) fn to_shortened_name(&self) -> &'static str {
        match self {
            ChipType::Burst => "BST",
            ChipType::Construct => "CNS",
            ChipType::Melee => "MLE",
            ChipType::Projectile => "PRJ",
            ChipType::Wave => "WVE",
            ChipType::Recovery => "RCV",
            ChipType::Summon => "SUM",
            ChipType::Support => "SPT",
            ChipType::Trap => "TRP",
        }
    }
}