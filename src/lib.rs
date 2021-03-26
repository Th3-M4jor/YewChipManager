#![recursion_limit = "1024"]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::eval_order_dependence)]
#![feature(try_trait, option_result_contains, try_blocks)]

pub mod app;
mod components;
mod chip_library;
mod util;
mod agents;

use wasm_bindgen::{
    prelude::*,
    JsCast,
};
use wasm_bindgen_futures::JsFuture;
use web_sys::Response;
use chip_library::ChipLibrary;
use app::App;

// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


const URL: &str = "https://spartan364.hopto.org/chips.json";

/// The entrypoint for the webapp
///
/// data is assumed to be the chips.json file's text
#[wasm_bindgen]
pub async fn run() -> Result<(), JsValue> {
    
    // Use a higher log level on debug
    #[cfg(debug_assertions)]
    wasm_logger::init(wasm_logger::Config::default());

    // only log errors on release builds
    #[cfg(not(debug_assertions))]
    wasm_logger::init(wasm_logger::Config::new(log::Level::Error));

    let window = web_sys::window().ok_or(wasm_bindgen::JsValue::from_str("could not retrive window, inform major"))?;
    
    let resp_value = JsFuture::from(window.fetch_with_str(URL)).await?;

    let resp = resp_value.dyn_into::<Response>().map_err(|_| wasm_bindgen::JsValue::from_str("error fetching chips, inform major"))?;

    let resp_text_val = JsFuture::from(resp.text()?).await?;

    let data = resp_text_val.as_string().ok_or(wasm_bindgen::JsValue::from_str("error fetching chips, inform major"))?;

    // deserialize the chip library before starting
    if let Err(why) = ChipLibrary::init(&data) {
        return Err(wasm_bindgen::JsValue::from_str(&why));
    }

    ChipLibrary::set_close_event_handler();
    
    yew::start_app::<App>();
    Ok(())
}

#[wasm_bindgen]
pub fn save_before_exit() -> Result<(), JsValue> {
    ChipLibrary::get_instance().save_data().map_err(|s| wasm_bindgen::JsValue::from_str(s))
}