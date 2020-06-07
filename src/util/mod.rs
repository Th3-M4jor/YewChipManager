pub mod timeout;

use yew::prelude::*;
use crate::chip_library::elements::Elements;

use unchecked_unwrap::UncheckedUnwrap;

pub fn generate_element_images(elem: &[Elements]) -> Html {
    
    html!{

        <span style="white-space: nowrap; display: inline-block">
        {
            elem.iter().map(|element| html!{ 
                <img src={element.to_img_url()} alt=""/>
            }).collect::<Html>()
        }
        </span>
    }
}

pub unsafe fn alert(msg: &str) {
    let window = web_sys::window().unchecked_unwrap();
    let _ = window.alert_with_message(msg);
}