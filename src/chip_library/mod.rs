mod battle_chip;
mod elements;
mod skills;
mod chip_type;
mod ranges;


pub(crate) use self::battle_chip::BattleChip;
pub(crate) use self::elements::Elements;
pub(crate) use self::chip_type::ChipType;
pub(crate) use self::ranges::Ranges;
pub(crate) use self::skills::Skills;

use std::collections::hash_map::HashMap;
use std::sync::RwLock;
use serde::Serialize;
use once_cell::sync::OnceCell;
use unchecked_unwrap::UncheckedUnwrap;

use std::sync::atomic::{Ordering, AtomicU32};
use std::sync::Arc;

#[derive(Serialize)]
pub(crate) struct PackChip {
    pub owned: u8,
    pub used: u8,
    #[serde(skip)]
    pub chip: Arc<BattleChip>,
}

#[derive(Serialize)]
pub(crate) struct FolderChip {
    pub name: String,
    pub used: bool,
    #[serde(skip)]
    pub chip: Arc<BattleChip>,
}

pub(crate) struct ChipLibrary {
    pub library: HashMap<String, Arc<BattleChip>>,
    pub pack: RwLock<HashMap<String, PackChip>>,
    pub folder: RwLock<Vec<FolderChip>>,
    pub chip_limit: AtomicU32,
}

static INSTANCE: OnceCell<ChipLibrary> = OnceCell::new();

impl ChipLibrary {

    pub(crate) fn init(data: String) {
         //initialize library
        INSTANCE.get_or_init(|| {
            ChipLibrary::import_local(&data)
        });
    }

    // undefined behavior if init has yet to be called
    pub(crate) fn get_instance() -> &'static ChipLibrary {
        unsafe { INSTANCE.get().unchecked_unwrap() }
    }

    /// if the chip does not exist, causes undefined behavior
    pub(crate) unsafe fn get_chip_unchecked(name: &str) -> &BattleChip {
        let lib = ChipLibrary::get_instance();
        lib.library.get(name).unchecked_unwrap()
    }

    fn import_local(data: &str) -> ChipLibrary {
        let mut chip_list: Vec<BattleChip> = serde_json::from_str::<Vec<BattleChip>>(&data).expect("Failed to deserialize library");
        let mut library: HashMap<String, Arc<BattleChip>> = HashMap::with_capacity(chip_list.len());
        while !chip_list.is_empty() {
            let chip = chip_list.pop().unwrap();
            library.insert(chip.name.clone(), Arc::new(chip));
        }
        let window = web_sys::window().expect("Could not get window");
        let storage = match window.local_storage().ok().flatten() {
            Some(storage) => storage,
            None => {
                let _ = window.alert_with_message("Local storage is not available, it is used to backup your folder and pack periodically");
                return ChipLibrary {
                    library,
                    pack: RwLock::new(HashMap::new()),
                    folder: RwLock::new(Vec::new()),
                    chip_limit: AtomicU32::new(12),
                };
            }
        };

        let pack = RwLock::new(ChipLibrary::load_pack(&storage, &library).unwrap_or_default());
        let folder = RwLock::new(ChipLibrary::load_folder(&storage, &library).unwrap_or_default());
        let chip_limit = AtomicU32::new(ChipLibrary::load_chip_limit(&storage).unwrap_or(12));


        ChipLibrary {
            library,
            pack,
            folder,
            chip_limit,
        }
        
    }

    /// load the pack from local storage
    fn load_pack(storage: &web_sys::Storage, library: &HashMap<String, Arc<BattleChip>>) -> Option<HashMap<String, PackChip>> {
        let pack_str: String = storage.get_item("pack").ok().flatten()?;
        //let mut map = serde_json::from_str::<HashMap<String, (u8,u8)>>(&pack_str).ok()?;
        let json = serde_json::from_str::<serde_json::Value>(&pack_str).ok()?;
        let map = json.as_object()?;

        let mut to_ret: HashMap<String, PackChip> = HashMap::new();

        for pack_chip in map.iter() {
            let owned = pack_chip.1["owned"].as_i64().unwrap() as u8;
            let used = pack_chip.1["used"].as_i64().unwrap() as u8;
            if let Some(chip) = library.get(pack_chip.0.as_str()) {
                to_ret.insert(pack_chip.0.clone(), PackChip{
                    owned,
                    used,
                    chip: Arc::clone(chip),
                });
            } else {
                ChipLibrary::warn_missing_pack(pack_chip.0.as_str(), owned, used);
            }
        }

        Some(to_ret)
    }

    /// load the folder from local storage
    fn load_folder(storage: &web_sys::Storage, library: &HashMap<String, Arc<BattleChip>>) -> Option<Vec<FolderChip>> {
        let folder_str: String = storage.get_item("folder").ok().flatten()?;
        let json = serde_json::from_str::<serde_json::Value>(&folder_str).ok()?;
        let fldr = json.as_array()?;
        let mut to_ret: Vec<FolderChip> = Vec::new();
        for folder_chip in fldr.iter() {
            let name = folder_chip["name"].as_str().unwrap();
            let used = folder_chip["used"].as_bool().unwrap();
            if let Some(chip) = library.get(name) {
                to_ret.push(
                    FolderChip{
                    name: name.to_owned(),
                    used,
                    chip: Arc::clone(chip),
                });
            } else {
                ChipLibrary::warn_missing_fldr(name, used);
            }
            
        }
        Some(to_ret)
    }

    /// load the folder size from local storage
    fn load_chip_limit(storage: &web_sys::Storage) -> Option<u32> {
        let limit_str: String = storage.get_item("chip_limit").ok().flatten()?;
        limit_str.parse::<u32>().ok()
    }

    fn warn_missing_pack(name: &str, owned: u8, used: u8) {

        let window = web_sys::window().expect("Could not get window");
        let _ = window.alert_with_message(&format!(
            "Your pack had a chip named \"{}\", this no longer exists in the library, you owned {} (of which {} were used)",
            name, 
            owned, 
            used
        ));
    }

    fn warn_missing_fldr(name: &str, used: bool) {
        let window = web_sys::window().expect("Could not get window");
        let used_unused = if used {"used"} else {"unused"};
        let _ = window.alert_with_message(&format!(
            "Your folder had a chip named \"{}\", this no longer exists in the library, you had it marked as: {}",
            name, 
            used_unused  
        ));
    }

    /// add a copy of a chip to the pack
    pub(crate) fn add_copy_to_pack(&self, name: &str) -> Option<u8> {
        
        let mut pack = self.pack.write().unwrap();

        if let Some(chip) = pack.get_mut(name) {
            chip.owned += 1;
            return Some(chip.owned);
        }
        //else not already in pack
        let lib_chip = self.library.get(name)?;
        pack.insert(name.to_owned(), PackChip{
            used: 0,
            owned: 1,
            chip: Arc::clone(lib_chip),
        });

        Some(1)
    }

    /// returned bool indicates if it was the last chip of that kind in the pack
    pub(crate) fn move_to_folder(&self, name: &str) -> Result<bool, &'static str> {
        let mut folder = self.folder.write().unwrap();
        let mut pack = self.pack.write().unwrap();
        if self.chip_limit.load(Ordering::Relaxed) as usize <= folder.len() {
            return Err("Your folder is full");
        }

        let chip = self.library.get(name).ok_or("No chip with that name exists")?;
        let pack_chip = pack.get_mut(name).ok_or("There are no coppies of that chip in your pack")?;

        if pack_chip.used >= pack_chip.owned {
            return Err("You do not have any unused coppies of that chip");
        }

        pack_chip.owned -= 1;
        let folder_chip = FolderChip {
            name: name.to_owned(),
            used: false,
            chip: Arc::clone(chip),
        };

        folder.push(folder_chip);
        drop(folder);
        if pack_chip.owned != 0 {
            return Ok(false);
        }
        //else it is zero
        drop(pack_chip);
        pack.remove(name);
        Ok(true)
    }

    /// returned bool indicates if it was used or not
    pub(crate) fn return_fldr_chip_to_pack(&self, index: usize) -> Result<bool, &'static str> {
        let mut folder = self.folder.write().unwrap();
        if folder.len() <= index {
            return Err("Index was out of bounds");
        }
        let fldr_chip = folder.remove(index);
        let mut pack = self.pack.write().unwrap();
        let used_incr = if fldr_chip.used {1} else {0};
        if let Some(pack_chip) = pack.get_mut(&fldr_chip.name) {
            pack_chip.owned += 1;
            pack_chip.used += used_incr;
        } else {
            //else no coppies already in pack
            let pack_chip = PackChip {
                owned: 1,
                used: used_incr,
                chip: fldr_chip.chip,
            };
            pack.insert(fldr_chip.name, pack_chip);
        }
        Ok(fldr_chip.used)
    }

    pub(crate) fn clear_folder(&self) -> usize {
        let mut folder = self.folder.write().unwrap();
        let mut pack = self.pack.write().unwrap();
        let returned_count = folder.len();
        for fldr_chip in folder.drain(..) {
            
            //number to add to the used_count
            let used_incr = if fldr_chip.used {1} else {0};

            if let Some(pack_chip) = pack.get_mut(&fldr_chip.name) {
                
                pack_chip.owned += 1;
                pack_chip.used += used_incr;

            } else {
                //else no coppies already in the pack
                let pack_chip = PackChip {
                    owned: 1,
                    used: used_incr,
                    chip: fldr_chip.chip,
                };
                pack.insert(fldr_chip.name, pack_chip);
            }
        }
        returned_count
    }

    pub(crate) fn jack_out(&self) -> u32 {
        let mut accumulator: u32 = 0;
        let mut folder = self.folder.write().unwrap();
        for chip in folder.iter_mut() {
            if chip.used {
                accumulator += 1;
                chip.used = false;
            }
        }
        drop(folder);
        let mut pack = self.pack.write().unwrap();
        for (_, chip) in pack.iter_mut() {
            accumulator += chip.used as u32;
            chip.used = 0;
        }
        accumulator
    }

}