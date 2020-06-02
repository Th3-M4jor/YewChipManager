pub mod timeout;

use yew::prelude::*;
use crate::chip_library::elements::Elements;

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