pub mod battle_chip;
pub mod elements;
pub mod skills;
pub mod chip_type;
pub mod ranges;

use std::collections::hash_map::HashMap;
use std::sync::RwLock;
use serde::{Serialize, Deserialize};
use once_cell::sync::OnceCell;
use battle_chip::BattleChip;

#[derive(Serialize, Deserialize)]
pub struct PackChip {
    owned: u32,
    used: u32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct FolderChip {
    name: String,
    used: bool,
}

pub struct ChipLibrary {
    pub library: HashMap<String, BattleChip>,
    pub pack: HashMap<String, PackChip>,
    pub folder: Vec<FolderChip>,
    pub chip_limit: u32,
}

impl ChipLibrary {
    pub fn import_local(data: &str) -> ChipLibrary {
        let mut chip_list: Vec<BattleChip> = serde_json::from_str::<Vec<BattleChip>>(&data).expect("Failed to deserialize library");
        let mut library: HashMap<String, BattleChip> = HashMap::with_capacity(chip_list.len());
        while !chip_list.is_empty() {
            let chip = chip_list.pop().unwrap();
            library.insert(chip.name.clone(), chip);
        }
        let window = web_sys::window().expect("Could not get window");
        let storage = match window.local_storage().ok().flatten() {
            Some(storage) => storage,
            None => {
                let _ = window.alert_with_message("Local storage is not available, it is used to backup your folder and pack periodically");
                return ChipLibrary {
                    library,
                    pack: HashMap::new(),
                    folder: Vec::new(),
                    chip_limit: 12,
                };
            }
        };

        let pack = ChipLibrary::load_pack(&storage).unwrap_or_default();
        let folder = ChipLibrary::load_folder(&storage).unwrap_or_default();
        let chip_limit = ChipLibrary::load_chip_limit(&storage).unwrap_or(12);


        let mut chip_library = ChipLibrary {
            library,
            pack,
            folder,
            chip_limit,
        };

        chip_library.check_missing_pack();
        chip_library.check_missing_folder();

        chip_library
        
    }

    /// load the pack from local storage
    fn load_pack(storage: &web_sys::Storage) -> Option<HashMap<String, PackChip>> {
        let pack_str: String = storage.get_item("pack").ok().flatten()?;
        serde_json::from_str::<HashMap<String, PackChip>>(&pack_str).ok()
    }

    /// load the folder from local storage
    fn load_folder(storage: &web_sys::Storage) -> Option<Vec<FolderChip>> {
        let folder_str: String = storage.get_item("folder").ok().flatten()?;
        serde_json::from_str::<Vec<FolderChip>>(&folder_str).ok()
    }

    /// load the folder size from local storage
    fn load_chip_limit(storage: &web_sys::Storage) -> Option<u32> {
        let limit_str: String = storage.get_item("chip_limit").ok().flatten()?;
        limit_str.parse::<u32>().ok()
    }

    /// check for if the pack has a chip that is not in the library
    fn check_missing_pack(&mut self) {
        let mut to_remove_from_pack = Vec::new();
        let window = web_sys::window().expect("Could not get window");
        for chip in self.pack.iter() {
            if !self.library.contains_key(chip.0) {
                let _ = window.alert_with_message(&format!(
                    "Your pack had a chip named \"{}\", this no longer exists in the library, you owned {} (of which {} were used)",
                    chip.0, 
                    chip.1.owned, 
                    chip.1.used
                ));
                to_remove_from_pack.push(chip.0.to_owned());
            }
        }

        // because you cannot borrow mutable while borrowing immutably

        for to_remove in to_remove_from_pack {
            self.pack.remove(&to_remove);
        }

    }

    /// check for if the folder has a chip that is not in the library
    fn check_missing_folder(&mut self) {
        let window = web_sys::window().expect("Could not get window");
        let mut new_folder = Vec::new();
        for chip in self.folder.iter() {
            if self.library.contains_key(&chip.name) {
                let used_unused = if chip.used {"used"} else {"unused"};
                let _ = window.alert_with_message(&format!(
                    "Your folder had a chip named \"{}\", this no longer exists in the library (You had it marked as {})",
                    chip.name,
                    used_unused
                ));
            } else {
                new_folder.push(chip.clone());
            }
        }
        
        self.folder = new_folder;
    }

    pub fn get_battlechip(&self, name: &str) -> Option<&BattleChip> {
        self.library.get(name)
    }
}

static INSTANCE: OnceCell<RwLock<ChipLibrary>> = OnceCell::new();

pub fn init_library(data: String) {
    
    //initialize library
    INSTANCE.get_or_init(|| {
        RwLock::new(ChipLibrary::import_local(&data))
    });

}

pub fn get_instance() -> &'static OnceCell<RwLock<ChipLibrary>> {
    &INSTANCE
}