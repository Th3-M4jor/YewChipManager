use yew::prelude::*;
use yew::services::interval::{IntervalService, IntervalTask};

use crate::chip_library::{ChipLibrary, BattleChip};
use crate::agents::chip_desc::{ChipDescMsgBus, ChipDescMsg};
use std::rc::Rc;
use std::time::Duration;

use unchecked_unwrap::UncheckedUnwrap;

pub(crate) enum ChipDescComponentMsg {
    SetDesc(String),
    ShowUnknown(String),
    ClearDesc,
    DoNothing,
}

pub(crate) struct ChipDescComponent {
    chip_anim_ct: usize,
    curr_chip: Option<Rc<BattleChip>>,
    _scroll_interval: IntervalTask,
    _link: ComponentLink<Self>,
    _producer: Box<dyn Bridge<ChipDescMsgBus>>,
}

impl Component for ChipDescComponent {
    type Message = ChipDescComponentMsg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.callback(|e| {
            match e {
                ChipDescMsg::SetDesc(name) => {
                    if ChipLibrary::get_instance().library.contains_key(&name) {
                        ChipDescComponentMsg::SetDesc(name)
                    } else {
                        ChipDescComponentMsg::ShowUnknown(name)
                    }
                },
                
                ChipDescMsg::ClearDesc => ChipDescComponentMsg::ClearDesc,
            }
        });
        let _producer = ChipDescMsgBus::bridge(callback);
        let _scroll_interval = IntervalService::spawn(Duration::from_millis(75), link.callback(scroll_interval));//unsafe{set_interval(75, scroll_interval).unchecked_unwrap()};

        Self {
            chip_anim_ct: 0,
            curr_chip: None,
            _scroll_interval,
            _link: link,
            _producer,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            ChipDescComponentMsg::SetDesc(name) => {
                self.set_chip(&name)
            }
            ChipDescComponentMsg::ShowUnknown(name) => {
                self.curr_chip = Some(Rc::new(BattleChip::unknown_chip(&name)));
                self.chip_anim_ct += 1;
                true
            }
            ChipDescComponentMsg::ClearDesc => {
                self.curr_chip.take();
                true
            }
            ChipDescComponentMsg::DoNothing => false,
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        match &self.curr_chip {
            Some(chip) => self.with_chip(chip.as_ref()),
            None => self.no_chip(),
        }

    }


    
}

impl ChipDescComponent {
    fn set_chip(&mut self, name: &str) -> bool {
        let new_chip = unsafe{ChipLibrary::get_instance().library.get(name).unchecked_unwrap()};
        match &self.curr_chip {
            Some(curr_chip) => {
                if Rc::ptr_eq(new_chip, curr_chip) {
                    return false;
                }
                self.curr_chip = Some(Rc::clone(new_chip));
                self.chip_anim_ct += 1;
                return true;
            }
            None => {
                self.curr_chip = Some(Rc::clone(new_chip));
                self.chip_anim_ct += 1;
                return true;
            }
        }
    }

    fn no_chip(&self) -> Html {
        html!{
            <div class="right-panel nopadding chipDescBackgroundStd"/>
        }
    }

    fn with_chip(&self, chip: &BattleChip) -> Html {
        let background = String::from("right-panel nopadding ") + chip.kind.to_background_css_class();
        let chip_anim_class = if self.chip_anim_ct & 1 == 0 {
            "chipWindowOne chipDescText chipDescPadding"
        } else {
            "chipWindowTwo chipDescText chipDescPadding"
        };
        let font_style = if chip.description.len() > 700 {
            "font-size: 12px; text-align: left"
        } else if chip.description.len() > 450 {
            "font-size: 14px; text-align: left"
        } else {
            "font-size: 16px; text-align: left"
        };

        html!{
            <div class=background>
                <div class=chip_anim_class style="padding: 3px">
                    {chip.damage_span()}
                    {chip.range_span()}
                    {chip.hits_span()}
                    <br/>
                    <div style={font_style} class="chipDescDiv" id="ScrollTextDiv">
                        {&chip.description}
                    </div>
                </div>
            </div>
        }
    }
}


fn scroll_interval(_: ()) -> ChipDescComponentMsg {
    let window = unsafe{web_sys::window().unchecked_unwrap()};
    let document = unsafe{window.document().unchecked_unwrap()};
    let elem = document.get_element_by_id("ScrollTextDiv");
    let div = match elem {
        Some(div) => div,
        None => return ChipDescComponentMsg::DoNothing,
    };
 
    let client_height = div.client_height();
    let total_height = div.scroll_height();
    let scroll_pos = div.scroll_top();
 
    let max_scroll = total_height - client_height;
 
    if max_scroll - 10 <= 0 {
        return ChipDescComponentMsg::DoNothing;
    }
 
    /*
    if scroll_pos == max_scroll {
        div.set_scroll_top(0);
        return;
    }
    */
 
    div.set_scroll_top(scroll_pos + 1);
    ChipDescComponentMsg::DoNothing
 }