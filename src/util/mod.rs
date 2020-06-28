use yew::prelude::*;
use wasm_bindgen::prelude::*;
use crate::chip_library::Elements;

use unchecked_unwrap::UncheckedUnwrap;

pub(crate) fn generate_element_images(elem: &[Elements]) -> Html {
    
    html!{

        <span class="chipImgBox">
        {
            /*
            elem.iter().map(|element| html!{ 
                <img src={element.to_img_url()} alt="" class="chipImg"/>
            }).collect::<Html>()
            */
            elem.iter().map(|element| html!{ 
                <span class={element.to_css_class()}/>
            }).collect::<Html>()
        }
        </span>
    }
}

pub unsafe fn alert(msg: &str) {
    let window = web_sys::window().unchecked_unwrap();
    let _ = window.alert_with_message(msg);
}

#[wasm_bindgen(module="/static/util.js")]
extern "C" {
    pub(crate) fn save_json(data: String);

    pub(crate) fn save_txt(data: String);

    pub(crate) fn storage_available(kind: String) -> bool;
}