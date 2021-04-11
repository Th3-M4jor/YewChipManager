use yew::prelude::*;
use yew_services::interval::{IntervalService, IntervalTask};

use crate::chip_library::{ChipLibrary, BattleChip};
use crate::agents::chip_desc::{ChipDescMsgBus, ChipDescMsg};
use std::rc::Rc;
use std::time::Duration;

use unchecked_unwrap::UncheckedUnwrap;

pub(crate) enum ChipDescComponentMsg {
    SetDesc(String),
    ShowUnknown(String),
    StopScroll,
    StartScroll,
    ClearDesc,
    DoNothing,
}

pub(crate) struct ChipDescComponent {
    chip_anim_ct: usize,
    curr_chip: Option<Rc<BattleChip>>,
    scroll_interval: Option<IntervalTask>,
    mouse_enter_event: Callback<MouseEvent>,
    mouse_leave_event: Callback<MouseEvent>,
    link: ComponentLink<Self>,
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
        let scroll_interval = IntervalService::spawn(Duration::from_millis(75), link.callback(scroll_interval));//unsafe{set_interval(75, scroll_interval).unchecked_unwrap()};
        let mouse_enter_event = link.callback(|_: MouseEvent| ChipDescComponentMsg::StopScroll);
        let mouse_leave_event = link.callback(|_: MouseEvent| ChipDescComponentMsg::StartScroll);
        Self {
            chip_anim_ct: 0,
            curr_chip: None,
            scroll_interval: Some(scroll_interval),
            mouse_enter_event,
            mouse_leave_event,
            link,
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
            ChipDescComponentMsg::StartScroll => {
                let interval = IntervalService::spawn(Duration::from_millis(75), self.link.callback(scroll_interval));
                self.scroll_interval = Some(interval);
                false
            },
            ChipDescComponentMsg::StopScroll => {
                self.scroll_interval.take();
                false
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
        let background = String::from("right-panel nopadding ") + chip.class.to_background_css_class();
        let chip_anim_class = if self.chip_anim_ct & 1 == 0 {
            "chipWindowOne"
        } else {
            "chipWindowTwo"
        };

        let font_style = if chip.description.len() > 700 {
            "chipDescSm" //"font-size: 12px; text-align: left; border-top: 1px solid black;"
        } else if chip.description.len() > 450 {
            "chipDescMd" //"font-size: 14px; text-align: left; border-top: 1px solid black;"
        } else {
            "chipDescLg" //"font-size: 16px; text-align: left; border-top: 1px solid black;"
        };

        let outer_chip_class = classes!("chipDescText", "chipDescPadding", chip_anim_class);
        let inner_chip_class = classes!(font_style, "chipDescDiv");
        let enter_clone = self.mouse_enter_event.clone();
        let leave_clone = self.mouse_leave_event.clone();
        html!{
            <div class=background onmouseover=enter_clone onmouseout=leave_clone>
                <div class=outer_chip_class style="padding: 3px; font-size: 14px;">
                    {chip.gen_desc_top_row()}
                    <div class=inner_chip_class id="ScrollTextDiv">
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