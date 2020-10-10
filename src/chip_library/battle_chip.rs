use crate::chip_library::{elements::Elements, skills::Skills, chip_type::ChipType, ranges::Ranges};
use serde::Deserialize;
use unchecked_unwrap::UncheckedUnwrap;
use std::cell::UnsafeCell;
use std::cmp::{Ord, Ordering};
use yew::prelude::*;

#[derive(Deserialize)]
#[serde(rename_all(deserialize = "PascalCase"))]
pub(crate) struct BattleChip {
    pub name: String,
    pub element: Vec<Elements>,
    pub skills: Vec<Skills>,
    pub damage: String,
    #[serde(rename(deserialize = "Type"))]
    pub kind: ChipType,
    pub range: Ranges,
    pub hits: String,
    pub description: String,
    #[serde(skip, default = "default_dmg_cell_float")]
    avg_dmg: UnsafeCell<Option<f32>>,
    #[serde(skip, default = "default_dmg_cell_int")]
    max_dmg: UnsafeCell<Option<u32>>,
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

impl BattleChip {
    pub(crate) fn skill(&self) -> Skills {
        if self.skills.len() > 1 {
            return Skills::Varies;
        }
        unsafe{self.skills.get(0).unchecked_unwrap()}.to_owned()
    }

    pub(crate) fn avg_dmg(&self) -> f32 {
        let val = unsafe {&*self.avg_dmg.get()};
        if let Some(avg) = val {
            return *avg;
        }
        //else
        drop(val);
        self.load_dmg().0
    }

    pub(crate) fn max_dmg(&self) -> u32 {
        let val = unsafe {&*self.max_dmg.get()};
        if let Some(max) = val {
            return *max;
        }
        //else
        drop(val);
        self.load_dmg().1
    }

    fn load_dmg(&self) -> (f32, u32) {
        
        let res = self.damage.split('d').collect::<Vec<&str>>();
        let (max, avg) = if res.len() != 2 {
            (0, 0.0)
        } else {
            let num_dice = unsafe{res.get_unchecked(0)}.parse::<u32>().unwrap_or(0);
            let die_size = unsafe{res.get_unchecked(1)}.parse::<u32>().unwrap_or(0);

            let avg = ((die_size as f32 / 2.0) + 0.5) * (num_dice as f32);
            let max = num_dice * die_size;
            (max, avg)
        };

        let (avg_ptr, max_ptr) = unsafe{(&mut *self.avg_dmg.get(), &mut *self.max_dmg.get())};

        *avg_ptr = Some(avg);
        *max_ptr = Some(max);

        (avg, max)
    }

    pub(crate) fn unknown_chip(name: &str) -> BattleChip {
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

    pub(crate) fn damage_span(&self) -> Html {
        if self.damage == "--" {
            html!{}
        } else {
            html!{
                <span style="float: left">{&self.damage}</span>
            }
        }
    }

    #[inline]
    pub(crate) fn range_span(&self) -> Html {
        html!{
            <span>{&self.range.as_str()}</span>
        }
    }

    pub(crate) fn hits_span(&self) -> Html {
        //else it's a range so we'll set it to a value that isn't 1 or 0, which are special cases
        let count = self.hits.parse::<isize>().unwrap_or(-1); 

        if count == 0 {
            html!{}
        } else if count == 1 {
            html!{
                <span style="float: right">{"1 hit"}</span>
            }
        } else {
            let mut text = String::from(&self.hits);
            text.push_str(" hits");
            html!{
                <span style="float: right">{text}</span>
            }
        }
    }

}

fn default_dmg_cell_float() -> UnsafeCell<Option<f32>> {
    UnsafeCell::new(None)
}

fn default_dmg_cell_int() -> UnsafeCell<Option<u32>> {
    UnsafeCell::new(None)
}