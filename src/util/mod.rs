pub(crate) mod timeout;

use yew::prelude::*;
use wasm_bindgen::prelude::*;
use crate::chip_library::Elements;

use unchecked_unwrap::UncheckedUnwrap;

pub(crate) fn generate_element_images(elem: &[Elements]) -> Html {
    
    html!{

        <span style="white-space: nowrap; display: inline-block">
        {
            elem.iter().map(|element| html!{ 
                <img src={element.to_img_url()} alt="" class="chipImg"/>
            }).collect::<Html>()
        }
        </span>
    }
}

pub unsafe fn alert(msg: &str) {
    let window = web_sys::window().unchecked_unwrap();
    let _ = window.alert_with_message(msg);
}

#[wasm_bindgen(module="/static/save.js")]
extern "C" {
    pub(crate) fn save_json(data: String);

    pub(crate) fn save_txt(data: String);
}