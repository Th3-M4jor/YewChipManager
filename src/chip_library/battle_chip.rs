use crate::chip_library::{elements::Elements, skills::Skills, chip_type::{ChipClass, ChipType}, ranges::Ranges};
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
    pub class: ChipClass,
    #[serde(rename(deserialize = "Type"))]
    pub kind: ChipType,
    pub range: Ranges,
    pub hits: String,
    pub description: String,
    #[serde(skip, default = "default_dmg_cell_float")]
    avg_dmg: UnsafeCell<f32>,
    #[serde(skip, default = "default_dmg_cell_int")]
    max_dmg: UnsafeCell<i32>,
}

// using Unsafe cell because shouldn't need to be a mutex
impl Clone for BattleChip {
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            element: self.element.clone(),
            skills: self.skills.clone(),
            damage: self.damage.clone(),
            class: self.class.clone(),
            kind: self.kind.clone(),
            range: self.range.clone(),
            hits: self.hits.clone(),
            description: self.description.clone(),
            avg_dmg: UnsafeCell::new(f32::NAN),
            max_dmg: UnsafeCell::new(-1),
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
        let val = unsafe {*self.avg_dmg.get()};
        if !val.is_nan() {
            return val;
        }
        //else
        self.load_dmg().0
    }

    pub(crate) fn max_dmg(&self) -> i32 {
        let val = unsafe {*self.max_dmg.get()};
        if val != -1 {
            return val;
        }
        //else
        self.load_dmg().1
    }

    fn load_dmg(&self) -> (f32, i32) {
        
        let res = self.damage.split('d').collect::<Vec<&str>>();
        let (max, avg) = if res.len() != 2 {
            (0, 0.0)
        } else {

            let dice_res: Option<(i32, i32)> = try {
                let num_dice = res.get(0)?.parse::<i32>().ok()?;
                let die_size = res.get(1)?.parse::<i32>().ok()?;
                (num_dice, die_size)
            };

            let (num_dice, die_size) = dice_res.unwrap_or((0,0));

            let avg = ((die_size as f32 / 2.0) + 0.5) * (num_dice as f32);
            let max = num_dice * die_size;
            (max, avg)
        };

        let (avg_ptr, max_ptr) = unsafe{(&mut *self.avg_dmg.get(), &mut *self.max_dmg.get())};

        *avg_ptr = avg;
        *max_ptr = max;

        (avg, max)
    }

    pub(crate) fn unknown_chip(name: &str) -> BattleChip {
        BattleChip {
            name: name.to_owned(),
            element: vec![Elements::Null],
            skills: vec![Skills::None],
            damage: "--".to_owned(),
            class: ChipClass::Dark,
            kind: ChipType::Burst,
            range: Ranges::Itself,
            hits: "--".to_string(),
            description: "Unknown Chip".to_owned(),
            avg_dmg: UnsafeCell::new(f32::NAN),
            max_dmg: UnsafeCell::new(-1),
        }
    }

    /*
    pub(crate) fn damage_span(&self) -> Html {
        if self.damage == "--" {
            html!{}
        } else {
            html!{
                <span style="float: left">{&self.damage}</span>
            }
        }
    }
    */

    pub(crate) fn gen_desc_top_row(&self) -> Html {
        match self.hits_span() {
            Some(hits) => {
                html!{
                    <div class="chip-row">
                        <div class="chip-col-3" style="border-right: 1px solid black">{self.kind.to_shortened_name()}</div>
                        <div class="chip-col-3">{&self.range.as_str()}</div>
                        {hits}
                    </div>
                }
            }
            None => {
                html!{
                    <div class="chip-row">
                        <div class="chip-col-3" style="border-right: 1px solid black">{self.kind.to_shortened_name()}</div>
                        <div class="chip-col-3">{&self.range.as_str()}</div>
                    </div>
                }
            }
        }
    }

    fn hits_span(&self) -> Option<Html> {
        //else it's a range so we'll set it to a value that isn't 1 or 0, which are special cases
        let count = self.hits.parse::<i32>().unwrap_or(-1);

        let html = if count == 0 {
            return None;
        } else if count == 1 {
            html!{
                <div class="chip-col-3" style="border-left: 1px solid black">{"1"}</div>
            }
        } else {
            let text = String::from(&self.hits);
            //text.push_str(" hits");
            html!{
                <div class="chip-col-3" style="border-left: 1px solid black">{text}</div>
            }
        };
        Some(html)
    }

}

fn default_dmg_cell_float() -> UnsafeCell<f32> {
    UnsafeCell::new(f32::NAN)
}

fn default_dmg_cell_int() -> UnsafeCell<i32> {
    UnsafeCell::new(-1)
}