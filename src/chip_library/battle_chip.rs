use crate::chip_library::{elements::Elements, skills::Skills, chip_type::ChipType, ranges::Ranges};
use serde::Deserialize;
use std::cmp::{Ord, Ordering};
#[derive(Deserialize)]
#[serde(rename_all(deserialize = "PascalCase"))]
pub struct BattleChip {
    pub name: String,
    pub element: Vec<Elements>,
    pub skills: Vec<Skills>,
    pub damage: String,
    #[serde(rename(deserialize = "Type"))]
    pub kind: ChipType,
    pub range: Ranges,
    pub hits: String,
    pub description: String,
}

impl Ord for BattleChip {
    fn cmp(&self, other: &Self) -> Ordering {
        self.name.to_ascii_lowercase().cmp(&other.name.to_ascii_lowercase())
    }
}

impl PartialOrd for BattleChip {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for BattleChip {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for BattleChip {}

impl BattleChip {
    pub fn skill(&self) -> Skills {
        if self.skills.len() > 1 {
            return Skills::Varies;
        }
        self.skills[0]
    }
}