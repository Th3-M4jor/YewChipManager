use crate::chip_library::{elements::Elements, skills::Skills, chip_type::ChipType, ranges::Ranges};
use serde::Deserialize;
use std::cell::UnsafeCell;
use std::cmp::{Ord, Ordering};
use once_cell::sync::Lazy;
use regex::Regex;

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
    #[serde(skip, default = "default_dmg_cell")]
    avg_dmg: UnsafeCell<Option<f32>>,
    #[serde(skip, default = "default_dmg_cell")]
    max_dmg: UnsafeCell<Option<f32>>,
}

// using Unsafe cell because shouldn't need to be a mutex
impl Clone for BattleChip {
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            element: self.element.clone(),
            skills: self.skills.clone(),
            damage: self.damage.clone(),
            kind: self.kind.clone(),
            range: self.range.clone(),
            hits: self.hits.clone(),
            description: self.description.clone(),
            avg_dmg: UnsafeCell::new(None),
            max_dmg: UnsafeCell::new(None),
        }
    }
}

unsafe impl Sync for BattleChip{}

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

static DAMAGE_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"(\d+)d(\d+)").unwrap());

impl BattleChip {
    pub fn skill(&self) -> Skills {
        if self.skills.len() > 1 {
            return Skills::Varies;
        }
        self.skills[0]
    }

    pub fn unknown_chip(name: &str) -> BattleChip {
        BattleChip {
            name: name.to_owned(),
            element: vec![Elements::Null],
            skills: vec![Skills::None],
            damage: "--".to_owned(),
            kind: ChipType::Dark,
            range: Ranges::Itself,
            hits: "--".to_string(),
            description: "Unknown Chip".to_owned(),
            avg_dmg: UnsafeCell::new(None),
            max_dmg: UnsafeCell::new(None),
        }
    }

    pub fn avg_dmg(&self) -> f32 {
        let val = unsafe {&*self.avg_dmg.get()};
        if let Some(avg) = val {
            return *avg;
        }
        //else
        drop(val);
        self.load_dmg().0
    }

    pub fn max_dmg(&self) -> f32 {
        let val = unsafe {&*self.max_dmg.get()};
        if let Some(max) = val {
            return *max;
        }
        //else
        drop(val);
        self.load_dmg().1
    }

    fn load_dmg(&self) -> (f32, f32) {
        let (max,avg) = match DAMAGE_REGEX.captures(&self.damage) {
            Some(damage_val) => {
                let num_dice: f32 = damage_val[1].parse::<f32>().unwrap();
                let die_size: f32 = damage_val[2].parse::<f32>().unwrap();

                let avg = ((die_size / 2.0) + 0.5) * num_dice;
                let max = num_dice * die_size;
                (max, avg)
            }
            None => (0.0, 0.0),
        };

        let (avg_ptr, max_ptr) = unsafe{(&mut *self.avg_dmg.get(), &mut *self.max_dmg.get())};

        *avg_ptr = Some(avg);
        *max_ptr = Some(max);

        (avg, max)
    }


}

fn default_dmg_cell() -> UnsafeCell<Option<f32>> {
    UnsafeCell::new(None)
}