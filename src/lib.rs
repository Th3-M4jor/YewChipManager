#![recursion_limit = "1024"]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::eval_order_dependence)]
#![feature(try_trait, option_result_contains)]

pub mod app;
mod components;
mod chip_library;
mod util;
mod agents;

use wasm_bindgen::prelude::*;

use chip_library::ChipLibrary;
use app::App;

// Use `wee_alloc` as the global allocator.
//#[global_allocator]
//static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// This is the entry point for the web app
/*
#[wasm_bindgen]
pub async fn run() -> Result<(), JsValue> {
    #[cfg(debug_assertions)]
    wasm_logger::init(wasm_logger::Config::default());
    #[cfg(not(debug_assertions))]
    wasm_logger::init(wasm_logger::Config::new(log::Level::Error));
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);
    let url = "https://spartan364.hopto.org/chips.json";
    let request = Request::new_with_str_and_init(&url, &opts).unwrap();
    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: Response = resp_value.dyn_into().unwrap();
    let data = JsFuture::from(resp.text()?).await.unwrap().as_string().unwrap();
    ChipLibrary::init(data);
    yew::start_app::<App>();
    Ok(())
}
*/

#[wasm_bindgen]
pub fn run(data: &str) -> Result<(), JsValue> {
    #[cfg(debug_assertions)]
    wasm_logger::init(wasm_logger::Config::default());
    #[cfg(not(debug_assertions))]
    wasm_logger::init(wasm_logger::Config::new(log::Level::Error));
    ChipLibrary::init(data);
    
    //fn to cache base64 values to improve performance potentially at the cost of memory usage
    //chip_library::Elements::intern_urls();
    yew::start_app::<App>();
    Ok(())
}

#[wasm_bindgen]
pub fn save_before_exit() -> Result<(), JsValue> {
    ChipLibrary::get_instance().save_data();
    Ok(())
}