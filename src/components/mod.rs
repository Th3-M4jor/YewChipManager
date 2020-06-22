pub(crate) mod library;
pub(crate) mod pack;
pub(crate) mod folder;
pub(crate) mod sort_box;
pub(crate) mod chips;
pub(crate) mod chip_desc;
pub(crate) mod group_folder;

#[derive(Eq, PartialEq, Clone, Copy)]
pub enum ChipSortOptions {
    Name,
    Element,
    MaxDamage,
    AverageDamage,
    Skill,
    Range,
    Owned,
}

/*
impl std::fmt::Display for ChipSortOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return match self {
            ChipSortOptions::Name => write!(f,"Name"),
            ChipSortOptions::Element => write!(f, "Element"),
            ChipSortOptions::MaxDamage => write!(f, "MaxDamage"),
            ChipSortOptions::AverageDamage => write!(f, "AverageDamage"),
            ChipSortOptions::Skill => write!(f, "Skill"),
            ChipSortOptions::Range => write!(f, "Range"),
            ChipSortOptions::Owned => write!(f, "Owned"),
        }
    }   
}
*/

impl ChipSortOptions {
    pub(crate) fn as_str(&self) -> &'static str {
        match self {
            ChipSortOptions::Name => {"Name"}
            ChipSortOptions::Element => {"Element"}
            ChipSortOptions::MaxDamage => {"MaxDmg"}
            ChipSortOptions::AverageDamage => {"AvgDmg"}
            ChipSortOptions::Skill => {"Skill"}
            ChipSortOptions::Range => {"Range"}
            ChipSortOptions::Owned => {"Owned"}
        }
    }
}

impl From<&str> for ChipSortOptions {
    fn from(val: &str) -> Self {
        match val {
            "Name" => ChipSortOptions::Name,
            "Element" => ChipSortOptions::Element,
            "MaxDamage" => ChipSortOptions::MaxDamage,
            "AverageDamage" => ChipSortOptions::AverageDamage,
            "Skill" => ChipSortOptions::Skill,
            "Range" => ChipSortOptions::Range,
            "Owned" => ChipSortOptions::Owned,
            _ => unreachable!(),
        }
    }
}