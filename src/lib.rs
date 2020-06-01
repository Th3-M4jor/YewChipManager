#![recursion_limit = "1024"]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::eval_order_dependence)]

pub mod app;
pub mod components;
pub mod chip_library;
mod util;

use wasm_bindgen::prelude::*;

use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

use app::App;

// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// This is the entry point for the web app
#[wasm_bindgen]
pub async fn run() -> Result<(), JsValue> {
    wasm_logger::init(wasm_logger::Config::default());
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);
    let url = "https://spartan364.hopto.org/chips.json";
    let request = Request::new_with_str_and_init(&url, &opts).unwrap();
    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: Response = resp_value.dyn_into().unwrap();
    let data = JsFuture::from(resp.text()?).await.unwrap().as_string().unwrap();
    chip_library::init_library(data);
    yew::start_app::<App>();
    Ok(())
}
