mod battle_chip;
mod elements;
mod skills;
mod chip_type;
mod ranges;


pub(crate) use self::battle_chip::BattleChip;
pub(crate) use self::elements::Elements;


use crate::util;
use std::{collections::hash_map::HashMap, ptr};
use std::cell::RefCell;
use serde::{Serialize, Deserialize};
use unchecked_unwrap::UncheckedUnwrap;
use serde_json::{Value, json};
use wasm_bindgen::{JsCast, JsValue, prelude::Closure};
use web_sys::BeforeUnloadEvent;
use std::sync::atomic::{Ordering, AtomicUsize, AtomicBool};
use std::rc::Rc;

#[derive(Serialize)]
pub(crate) struct PackChip {
    pub owned: u32,
    pub used: u32,
    #[serde(skip)]
    pub chip: Rc<BattleChip>,
}

#[derive(Serialize)]
pub(crate) struct FolderChip {
    pub name: String,
    pub used: bool,
    #[serde(skip)]
    pub chip: Rc<BattleChip>,
}

#[derive(Deserialize, Serialize, Clone)]
pub(crate) struct GroupFolderChip {
    pub name: String,
    pub used: bool,
}

pub(crate) struct ChipLibrary {
    pub library: HashMap<String, Rc<BattleChip>>,
    pub pack: RefCell<HashMap<String, PackChip>>,
    pub folder: RefCell<Vec<FolderChip>>,
    pub group_folders: RefCell<HashMap<String, Vec<GroupFolderChip>>>,
    pub chip_limit: AtomicUsize,
    pub close_event_handler: Option<Closure<dyn Fn(BeforeUnloadEvent) -> Result<JsValue, JsValue>>>,
    change_since_last_save: AtomicBool,
    change_since_last_group_post: AtomicBool,
}

unsafe impl Send for ChipLibrary{}
unsafe impl Sync for ChipLibrary{}

// using a ptr and allocating at runtime instead of an Option to reduce executable size
static mut INSTANCE: *mut ChipLibrary = ptr::null_mut();

impl ChipLibrary {

    pub(crate) fn init(data: &str) -> Result<(), String> {
        //initialize library, and allocate
        let library = Box::new(ChipLibrary::import_local(data)?);

        unsafe {
            INSTANCE = Box::into_raw(library); //convert to raw ptr, don't have to worry about deallocating later since it's basically static
        };
        Ok(())
    }

    /// undefined behavior if init has yet to be called
    pub(crate) fn get_instance() -> &'static ChipLibrary {
        unsafe { &*INSTANCE }
    }

    pub(crate) fn set_close_event_handler() {
        let handler: Closure<dyn Fn(BeforeUnloadEvent) -> Result<JsValue, JsValue>> = Closure::wrap(Box::new(|_: BeforeUnloadEvent| {
            ChipLibrary::get_instance().save_data().map_err(|s| wasm_bindgen::JsValue::from_str(s))?;
            Ok(wasm_bindgen::JsValue::from_str("Progress might be lost if you leave without saving an export."))
        }) as Box<dyn Fn(BeforeUnloadEvent) -> Result<JsValue, JsValue>>);

        let window = match web_sys::window() {
            Some(window) => window,
            None => return,
        };

        window.set_onbeforeunload(Some(handler.as_ref().unchecked_ref()));

        unsafe {
            (*INSTANCE).close_event_handler = Some(handler);
        }

    }

    fn import_local(data: &str) -> Result<ChipLibrary, String> {
        let mut chip_list: Vec<BattleChip> = match serde_json::from_str::<Vec<BattleChip>>(data) {
            Ok(chip_list) => chip_list,
            Err(e) => {
                let msg = e.to_string();
                unsafe{util::alert(&msg)};
                return Err(msg)
            }
        };
        let mut library: HashMap<String, Rc<BattleChip>> = HashMap::with_capacity(chip_list.len());
        for chip in chip_list.drain(..) {
            library.insert(chip.name.clone(), Rc::new(chip));
        }
        /*
        while !chip_list.is_empty() {
            let chip = chip_list.pop().unwrap();
            library.insert(chip.name.clone(), Rc::new(chip));
        }
        */

        let window = unsafe{web_sys::window().unchecked_unwrap()};
        let storage = match window.local_storage().ok().flatten() {
            Some(storage) => storage,
            None => {
                unsafe{util::alert("Local storage is not available, it is used to backup your folder and pack periodically")};
                return Ok(ChipLibrary {
                    library,
                    pack: RefCell::new(HashMap::new()),
                    folder: RefCell::new(Vec::new()),
                    chip_limit: AtomicUsize::new(12),
                    group_folders: RefCell::new(HashMap::new()),
                    change_since_last_save: AtomicBool::new(false),
                    change_since_last_group_post: AtomicBool::new(false),
                    close_event_handler: None,
                });
            }
        };

        let pack = RefCell::new(ChipLibrary::load_pack(&storage, &library).unwrap_or_default());
        let folder = RefCell::new(ChipLibrary::load_folder(&storage, &library).unwrap_or_default());
        let chip_limit = AtomicUsize::new(ChipLibrary::load_chip_limit(&storage).unwrap_or(12));

        Ok(ChipLibrary {
            library,
            pack,
            folder,
            chip_limit,
            group_folders: RefCell::new(HashMap::new()),
            change_since_last_save: AtomicBool::new(false),
            change_since_last_group_post: AtomicBool::new(false),
            close_event_handler: None,
        })
        
    }

    /// load the pack from local storage
    fn load_pack(storage: &web_sys::Storage, library: &HashMap<String, Rc<BattleChip>>) -> Option<HashMap<String, PackChip>> {
        let pack_str: String = storage.get_item("pack").ok().flatten()?; //flatten an Option<Option<String>> into Option<String>
        //let mut map = serde_json::from_str::<HashMap<String, (u8,u8)>>(&pack_str).ok()?;
        let json = serde_json::from_str::<serde_json::Value>(&pack_str).ok()?;
        let map = json.as_object()?;

        let mut to_ret: HashMap<String, PackChip> = HashMap::new();

        for pack_chip in map.iter() {
            let owned = unsafe{pack_chip.1["owned"].as_u64().unchecked_unwrap()} as u32;
            let used = unsafe{pack_chip.1["used"].as_u64().unchecked_unwrap()} as u32;
            if let Some(chip) = library.get(pack_chip.0.as_str()) {
                to_ret.insert(pack_chip.0.clone(), PackChip{
                    owned,
                    used,
                    chip: Rc::clone(chip),
                });
            } else {
                ChipLibrary::warn_missing_pack(pack_chip.0.as_str(), owned, used);
            }
        }

        Some(to_ret)
    }

    /// load the folder from local storage
    fn load_folder(storage: &web_sys::Storage, library: &HashMap<String, Rc<BattleChip>>) -> Option<Vec<FolderChip>> {
        let folder_str: String = storage.get_item("folder").ok().flatten()?; //flatten an Option<Option<String>> into Option<String>
        let json = serde_json::from_str::<serde_json::Value>(&folder_str).ok()?;
        let fldr = json.as_array()?;
        let mut to_ret: Vec<FolderChip> = Vec::new();
        for folder_chip in fldr.iter() {
            let name = unsafe{folder_chip["name"].as_str().unchecked_unwrap()};
            let used = unsafe{folder_chip["used"].as_bool().unchecked_unwrap()};
            if let Some(chip) = library.get(name) {
                to_ret.push(
                    FolderChip{
                    name: name.to_owned(),
                    used,
                    chip: Rc::clone(chip),
                });
            } else {
                ChipLibrary::warn_missing_fldr(name, used);
            }
            
        }
        Some(to_ret)
    }

    /// load the folder size from local storage
    fn load_chip_limit(storage: &web_sys::Storage) -> Option<usize> {
        let limit_str: String = storage.get_item("chip_limit").ok().flatten()?;
        limit_str.parse::<usize>().ok()
    }

    fn warn_missing_pack(name: &str, owned: u32, used: u32) {

        let mut msg = String::from("Your pack had a chip named \"");
        msg.push_str(name);
        msg.push_str("\", this no longer exists in the library, you owned ");
        msg.push_str(&owned.to_string());
        msg.push_str(" (of which ");
        msg.push_str(&used.to_string());
        msg.push_str(" were used)");
        unsafe{util::alert(&msg)};
    }

    fn warn_missing_fldr(name: &str, used: bool) {
        let used_unused = if used {"used"} else {"unused"};
        let mut msg = String::from("Your folder had a chip named \"");
        msg.push_str(name);
        msg.push_str("\", this no longer exists in the library, you had it marked as: ");
        msg.push_str(used_unused);
        unsafe{util::alert(&msg)};
    }

    /// add a copy of a chip to the pack
    pub(crate) fn add_copy_to_pack(&self, name: &str) -> Option<u32> {
        
        let mut pack = self.pack.try_borrow_mut().ok()?;

        if let Some(chip) = pack.get_mut(name) {
            chip.owned += 1;
            return Some(chip.owned);
        }
        //else not already in pack
        let lib_chip = self.library.get(name)?;
        pack.insert(name.to_owned(), PackChip{
            used: 0,
            owned: 1,
            chip: Rc::clone(lib_chip),
        });
        self.change_since_last_save.store(true, Ordering::Relaxed);
        Some(1)
    }

    /// returned bool indicates if it was the last chip of that kind in the pack
    pub(crate) fn move_to_folder(&self, name: &str) -> Result<bool, &'static str> {
        let folder = self.folder.try_borrow_mut();
        let pack = self.pack.try_borrow_mut();
        let (mut folder, mut pack) = match (folder,pack) {
            (Ok(folder), Ok(pack)) => (folder,pack),
            _ => {return Err("failed to borrow folder or pack, inform Major")}
        };
        if self.chip_limit.load(Ordering::Relaxed) <= folder.len() {
            return Err("Your folder is full");
        }

        let chip = self.library.get(name).ok_or("No chip with that name exists")?;
        let pack_chip = pack.get_mut(name).ok_or("There are no copies of that chip in your pack")?;

        if pack_chip.used >= pack_chip.owned {
            return Err("You do not have any unused copies of that chip");
        }

        // find out how many copies of that chip are in their folder
        let mut count = 0u8;

        for chip in folder.iter() {
            // can use ptr_eq of the pointed library chip since there will always be only one
            if Rc::ptr_eq(&chip.chip, &pack_chip.chip) {
                count += 1;
            }
        }

        // is it greater than the limit for that type of chip?
        if count >= pack_chip.chip.kind.max_in_folder() {
            return Err("You cannot add any more copies of that chip to your folder");
        }

        pack_chip.owned -= 1;
        let folder_chip = FolderChip {
            name: name.to_owned(),
            used: false,
            chip: Rc::clone(chip),
        };

        folder.push(folder_chip);
        drop(folder);
        self.change_since_last_save.store(true, Ordering::Relaxed);
        self.change_since_last_group_post.store(true, Ordering::Relaxed);
        if pack_chip.owned != 0 {
            return Ok(false);
        }
        //else it is zero
        drop(pack_chip);
        pack.remove(name);
        Ok(true)
    }

    pub(crate) fn flip_used_folder(&self, idx: usize) {
        let mut folder = match self.folder.try_borrow_mut() {
            Ok(folder) => folder,
            Err(_) => return,
        };
        let chip = match folder.get_mut(idx) {
            Some(chip) => chip,
            None => return,
        };
        chip.used = !chip.used;
        self.change_since_last_group_post.store(true, Ordering::Relaxed);
        self.change_since_last_save.store(true, Ordering::Relaxed);
    }

    /// returned bool indicates if it was the last chip of that kind in the pack
    pub(crate) fn remove_from_pack(&self, name:&str) -> Result<bool, &'static str> {
        let mut pack = match self.pack.try_borrow_mut() {
            Ok(pack) => pack,
            Err(_) => return Err("Failed to borrow pack mutably, inform Major"),
        };
        let pack_chip = pack.get_mut(name).ok_or("No chip with that name in the pack")?;
        pack_chip.owned -= 1;
        if pack_chip.owned != 0 {
            return Ok(false);
        }

        //else last chip
        drop(pack_chip);
        pack.remove(name);
        self.change_since_last_save.store(true, Ordering::Relaxed);
        Ok(true)
    }

    pub(crate) fn mark_pack_copy_unused(&self, name: &str) -> Result<u32, &'static str> {
        let mut pack = match self.pack.try_borrow_mut() {
            Ok(pack) => pack,
            Err(_) => return Err("Failed to borrow pack mutably, inform Major"),
        };
        let chip = pack.get_mut(name).ok_or("No copy of that chip in your pack")?;
        if chip.used == 0 {
            return Err("No used copies of that chip in you pack");
        }
        chip.used -= 1;
        self.change_since_last_save.store(true, Ordering::Relaxed);
        Ok(chip.used)
    }
    /// returned bool indicates if it was used or not
    pub(crate) fn return_fldr_chip_to_pack(&self, index: usize) -> Result<bool, &'static str> {
        let pack = self.pack.try_borrow_mut();
        let folder = self.folder.try_borrow_mut();

        let (mut folder, mut pack) = match (folder,pack) {
            (Ok(folder), Ok(pack)) => (folder,pack),
            _ => {return Err("failed to borrow folder or pack, inform Major")}
        };

        if folder.len() <= index {
            return Err("Index was out of bounds");
        }

        let fldr_chip = folder.remove(index);
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
        self.change_since_last_save.store(true, Ordering::Relaxed);
        self.change_since_last_group_post.store(true, Ordering::Relaxed);
        Ok(fldr_chip.used)
    }

    pub(crate) fn clear_folder(&self) -> usize {
        let pack = self.pack.try_borrow_mut();
        let folder = self.folder.try_borrow_mut();

        let (mut folder, mut pack) = match (folder,pack) {
            (Ok(folder), Ok(pack)) => (folder,pack),
            _ => return 0,
        };
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
        if returned_count > 0 {
            self.change_since_last_save.store(true, Ordering::Relaxed);
            self.change_since_last_group_post.store(true, Ordering::Relaxed);
        }
        returned_count
    }

    pub(crate) fn jack_out(&self) -> u32 {
        let mut accumulator: u32 = 0;
        let mut folder = match self.folder.try_borrow_mut() {
            Ok(folder) => folder,
            Err(_) => {
                unsafe{util::alert("failed to borrow folder mutably, inform Major")};
                return 0;
            }
        };

        for chip in folder.iter_mut() {
            if chip.used {
                accumulator += 1;
                chip.used = false;
            }
        }
        drop(folder);
        let mut pack = match self.pack.try_borrow_mut() {
            Ok(pack) => pack,
            Err(_) => {
                unsafe{util::alert("failed to borrow pack mutably, inform Major")};
                return 0;
            }
        };
        for (_, chip) in pack.iter_mut() {
            accumulator += chip.used as u32;
            chip.used = 0;
        }

        if accumulator > 0 {
            self.change_since_last_save.store(true, Ordering::Relaxed);
            self.change_since_last_group_post.store(true, Ordering::Relaxed);
        }

        accumulator
    }

    /// update the chip limit, returns true if the value changed
    pub(crate) fn update_chip_limit(&self, new_limit: usize) -> Result<bool, &'static str> {
        
        let folder = match self.folder.try_borrow() {
            Ok(folder) => folder,
            Err(_) => return Err("failed to borrow folder, inform Major"),
        };

        
        if new_limit < folder.len() {
            return Err("You must remove chips from your folder first");
        }

        if new_limit == self.chip_limit.load(Ordering::Relaxed) {
            return Ok(false);
        }

        self.chip_limit.store(new_limit, Ordering::Relaxed);
        self.change_since_last_save.store(true, Ordering::Relaxed);
        Ok(true)
    }

    pub(crate) fn export_json(&self) {
        let (folder, pack) = unsafe {
            let folder = self.folder.try_borrow_unguarded().unchecked_unwrap();
            let pack = self.pack.try_borrow_unguarded().unchecked_unwrap();
            (folder, pack)
        };
        let limit = self.chip_limit.load(Ordering::Relaxed);
        let to_save = json!({
            "Folder": folder,
            "Pack": pack,
            "Limit": limit,
        });
        let data = serde_json::to_string_pretty(&to_save).unwrap();
        crate::util::save_json(data);
    }

    pub(crate) fn import_json(&self, data: String) -> Result<(), &'static str> {
        self.erase_data();
        let save_data = serde_json::from_str::<Value>(&data).map_err(|_| "Ill formed save data")?;
        let limit = save_data["Limit"].as_u64().ok_or("Ill formed save data")?;
        if limit > 45 {
            return Err("Ill formed save data, folder limit too high");
        }
        self.chip_limit.store(limit as usize, Ordering::Relaxed);
        if let Some(folder_chips) = save_data["Folder"].as_array() {
            self.parse_folder(folder_chips)?;
        }
        if let Some(pack_chips) = save_data["Pack"].as_object() {
            self.parse_pack(pack_chips)?;
        }
        self.change_since_last_save.store(true, Ordering::Relaxed);
        Ok(())
    }

    fn parse_pack(&self, data: &serde_json::Map<String, Value>) -> Result<(), &'static str> {
        let mut pack = match self.pack.try_borrow_mut() {
            Ok(pack) => pack,
            Err(_) => return Err("failed to borrow pack mutably, inform Major"),
        };

        for (name, chip) in data.iter() {
            let owned = chip["owned"].as_u64().ok_or("Ill formed save data")?;
            let used = chip["used"].as_u64().ok_or("Ill formed save data")?;
            if used > owned {
                return Err("Ill formed save data");
            }
            if let Some(lib_chip) = self.library.get(name) {
                let pack_chip = PackChip{
                    owned: owned as u32,
                    used: used as u32,
                    chip: Rc::clone(lib_chip)
                };
                pack.insert(name.clone(), pack_chip);
            } else {
                let msg = String::from("Ignoring a chip your pack has that doesn't exist anymore: ") + name;
                unsafe{crate::util::alert(&msg)};
            }
        }
        Ok(())
    }

    fn parse_folder(&self, data: &Vec<serde_json::Value>) -> Result<(), &'static str> {
        let mut folder = match self.folder.try_borrow_mut() {
            Ok(folder) => folder,
            Err(_) => return Err("Failed to borrow folder mutably, inform Major"),
        };

        if data.len() > self.chip_limit.load(Ordering::Relaxed) as usize {
            return Err("Chip limit was set lower than the actual folder size");
        }

        for chip in data.iter() {
            let name = chip["name"].as_str().ok_or("Ill formed save data")?;
            let used = chip["used"].as_bool().ok_or("Ill formed save data")?;
            if let Some(lib_chip) = self.library.get(name) {
                folder.push(FolderChip{
                    name: name.to_owned(),
                    used,
                    chip: Rc::clone(lib_chip)
                });
            } else {
                let msg = String::from("Ignoring a chip your folder has that doesn't exist anymore: ") + name;
                unsafe{crate::util::alert(&msg)};
            }
        }
        Ok(())
    }

    pub(crate) fn save_data(&self) -> Result<(), &'static str> {
        if self.change_since_last_save.load(Ordering::Relaxed) == false {
            return Ok(());
        }
        
        let window = web_sys::window().ok_or("failed to get window")?;
        let storage = match window.local_storage().ok().flatten() {
            Some(storage) => storage,
            None => return Err("could not get storage"),
        };

        let pack = match self.pack.try_borrow() {
            Ok(pack) => pack,
            Err(_) => return Err("Failed to borrow pack, inform Major"),
        };

        let pack_text = serde_json::to_string(&*pack).map_err(|_|"Failed to serialize pack")?;
        storage.set_item("pack", &pack_text).unwrap();
        // no longer needed, free memory
        drop(pack_text);
        drop(pack);

        let folder = match self.folder.try_borrow() {
            Ok(folder) => folder,
            Err(_) => return Err("Failed to borrow folder, inform Major"),
        };

        //have to deref then borrow to coerce to a reference from a std::cell::Ref
        let folder_text = match serde_json::to_string(&*folder) {
            Ok(folder_text) => folder_text,
            Err(_) => return Err("Failed to serialize folder"),
        };//.map_err(|_| "Failed to serialize folder")?;

        storage.set_item("folder", &folder_text).unwrap();

        drop(folder);
        drop(folder_text);

        let chip_limit = self.chip_limit.load(Ordering::Relaxed).to_string();
        storage.set_item("chip_limit", &chip_limit).unwrap();

        self.change_since_last_save.store(false, Ordering::Relaxed);
        Ok(())
    }

    pub(crate) fn export_txt(&self) {
        let folder = self.folder.try_borrow();
        let pack = self.pack.try_borrow();

        let (folder, pack) = match (folder, pack) {
            (Ok(folder), Ok(pack)) => (folder, pack),
            _ => {
                unsafe{util::alert("failed to borrow folder or pack, inform Major")};
                return;
            }
        };

        //let mut to_save_text = String::with_capacity(100);
        let folder_text_vec = folder.iter().map(|chip| {
            let mut to_ret = String::from(&chip.name);
            if chip.used {
                to_ret.push_str(" (Used)");
            }
            to_ret
        }).collect::<Vec<String>>();
        let mut text_to_save = if folder_text_vec.len() > 0 {
            String::from("Folder: ") + &folder_text_vec.join(", ") + "\n"
        } else {
            String::new()
        };
        let mut pack_vec = pack.values().collect::<Vec<&PackChip>>();
        pack_vec.sort_unstable_by(|a,b| {
            a.chip.kind.cmp(&b.chip.kind).then_with(||a.chip.name.cmp(&b.chip.name))
        });

        let pack_str_vec = pack_vec.iter().map(|chip| {
            let mut to_ret = String::from(&chip.chip.name);
            if chip.owned == 1 && chip.used == 1 {
                to_ret += " (Used)";
                return to_ret;
            }
            
            if chip.owned > 1 {
                to_ret.push_str(" x");
                to_ret.push_str(&chip.owned.to_string());
            }

            if chip.used > 0 {
                to_ret.push_str(" (");
                to_ret.push_str(&chip.owned.to_string());
                to_ret.push_str(" Used)");
            }
            to_ret
        }).collect::<Vec<String>>();
        let pack_text = if pack_str_vec.len() > 0 {
            String::from("Pack: ") + &pack_str_vec.join(", ")
        } else {
            String::new()
        };
        text_to_save.push_str(&pack_text);
        crate::util::save_txt(text_to_save);
    }

    pub(crate) fn erase_data(&self) {
        self.chip_limit.store(12, Ordering::Relaxed);
        let folder = self.folder.try_borrow_mut();
        let pack = self.pack.try_borrow_mut();

        let (mut folder, mut pack) = match (folder, pack) {
            (Ok(folder), Ok(pack)) => (folder, pack),
            _ => {
                unsafe{util::alert("failed to borrow folder or pack, inform Major")};
                return;
            }
        };

        folder.clear();
        pack.clear();
        drop(folder);
        drop(pack);

        let window = match web_sys::window() {
            Some(window) => window,
            None => return
        };
        let storage = match window.local_storage().ok().flatten() {
            Some(storage) => storage,
            None => return
        };
        
        let _ = storage.remove_item("folder");
        let _ = storage.remove_item("pack");
        let _ = storage.remove_item("chip_limit");
        self.change_since_last_save.store(false, Ordering::Relaxed);
    }

    pub(crate) fn serialize_folder(&self) -> Vec<u8> {
        let folder = unsafe{self.folder.try_borrow().unchecked_unwrap()};
        let folder_str = bincode::serialize(&*folder).unwrap_or_else(|_| {
            let chips: Vec<FolderChip> = Vec::new();
            unsafe{bincode::serialize(&chips).unchecked_unwrap()}
        });
        self.change_since_last_group_post.store(false, Ordering::Relaxed);
        folder_str
    }
    
    /// Returns true if the folder has changed since the last time
    /// it was serialized for sharing via websocket
    #[inline]
    pub(crate) fn folder_changed(&self) -> bool {
        self.change_since_last_group_post.load(Ordering::Relaxed)
    }

    pub(crate) fn not_in_group_or_empty_fldr(&self, name: &str) -> bool {
        let folders = match self.group_folders.try_borrow() {
            Ok(folders) => folders,
            Err(_) => return true,
        };
        let player_folder = match folders.get(name) {
            Some(folder) => folder,
            None => return true,
        };
        player_folder.is_empty()
    }
}