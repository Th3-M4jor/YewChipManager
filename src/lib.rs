#![recursion_limit = "1024"]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::eval_order_dependence)]
#![feature(try_trait, option_result_contains, try_blocks)]

pub mod app;
mod components;
mod chip_library;
mod util;
mod agents;

use wasm_bindgen::prelude::*;

use chip_library::ChipLibrary;
use app::App;

// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/// The entrypoint for the webapp
///
/// data is assumed to be the chips.json file's text
#[wasm_bindgen]
pub fn run(data: &str) -> Result<(), JsValue> {
    
    // Use a higher log level on debug
    #[cfg(debug_assertions)]
    wasm_logger::init(wasm_logger::Config::default());

    // only log errors on release builds
    #[cfg(not(debug_assertions))]
    wasm_logger::init(wasm_logger::Config::new(log::Level::Error));

    // deserialize the chip library before starting
    if let Err(why) = ChipLibrary::init(data) {
        return Err(wasm_bindgen::JsValue::from_str(&why));
    }
    
    yew::start_app::<App>();
    Ok(())
}

#[wasm_bindgen]
pub fn save_before_exit() -> Result<(), JsValue> {
    ChipLibrary::get_instance().save_data().map_err(|s| wasm_bindgen::JsValue::from_str(s))
}