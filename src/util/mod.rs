use yew::prelude::*;
use wasm_bindgen::prelude::*;
use crate::chip_library::Elements;
use crate::ChipLibrary;

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
                <span class=element.to_css_class()/>
            }).collect::<Html>()
        }
        </span>
    }
}

pub(crate) fn list_spectators() -> Html {


    let folders = ChipLibrary::get_instance().group_folders.borrow();
    if folders.is_empty() {
        return html!{};
    }


    html!{
        <>
        <div class="Chip noselect" style="text-decoration: underline;">{"Spectators"}</div>
        {
        folders.iter().map(|folder| {
            if folder.1.is_empty() {
            
                let name = if folder.0.len() > 7 {
                    let mut to_ret = String::from(unsafe{folder.0.get_unchecked(..=4)});
                    to_ret.push_str("...");
                    to_ret
                } else {
                    folder.0.to_owned()
                };
            
                html!{
                    <div class="Chip">{name}</div>
                }
            } else {
                html!{}
            }
        }).collect::<Html>()
        }
        </>
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