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

///main is the entrypoint now
pub fn main() {
    
    // Use a higher log level on debug
    #[cfg(debug_assertions)]
    wasm_logger::init(wasm_logger::Config::default());

    // only log errors on release builds
    #[cfg(not(debug_assertions))]
    wasm_logger::init(wasm_logger::Config::new(log::Level::Error));
    
    wasm_bindgen_futures::spawn_local(async_main())

    //let promise = wasm_bindgen_futures::future_to_promise(start());
    //Ok(JsValue::from(promise))
    
}

async fn async_main() {

    let data = match call_fetch().await {
        Ok(data) => data,
        Err(why) => {
            web_sys::console::error_1(&why);
            unsafe{util::alert("An error occurred fetching chips, inform major")};
            return;
        }
    };

    // deserialize the chip library before starting
    if let Err(why) = ChipLibrary::init(&data) {
        web_sys::console::error_1(&wasm_bindgen::JsValue::from_str(&why));
        unsafe{util::alert("An error occurred deserializing chips, inform Major")};
        return;
    }

    ChipLibrary::set_close_event_handler();
    
    yew::start_app::<App>();

}

async fn call_fetch() -> Result<String, JsValue> {
    let window = web_sys::window().ok_or(wasm_bindgen::JsValue::from_str("could not retrive window, inform major"))?;
    
    let resp_value = JsFuture::from(window.fetch_with_str(URL)).await?;

    let resp = resp_value.dyn_into::<Response>().map_err(|_| wasm_bindgen::JsValue::from_str("error fetching chips, inform major"))?;

    let resp_text_val = JsFuture::from(resp.text()?).await?;

    resp_text_val.as_string().ok_or(wasm_bindgen::JsValue::from_str("error fetching chips, inform major"))
}

#[wasm_bindgen]
pub fn save_before_exit() -> Result<(), JsValue> {
    ChipLibrary::get_instance().save_data().map_err(|s| wasm_bindgen::JsValue::from_str(s))
}