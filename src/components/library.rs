use yew::prelude::*;
use yew::agent::{Dispatcher, Dispatched};
use yew::html::{ChangeData, InputData};
use yewtil::function_component;
use std::rc::Rc;
use unchecked_unwrap::UncheckedUnwrap;
use wasm_bindgen::JsCast;

use crate::components::{ChipSortOptions, chips::LibraryChip, sort_box::ChipSortBox};
use crate::chip_library::{BattleChip, ChipLibrary};
use crate::agents::chip_desc::{ChipDescMsg, ChipDescMsgBus};
use crate::util::list_spectators;



#[function_component(LibraryTopRow)]
pub(crate) fn library_top_row() -> Html {
    html! {
        <div class="chip-top-row noselect">
            <div class="chip-col-4 Chip nopadding" style="white-space: nowrap">
                {"NAME"}
            </div>
            <div class="chip-col-1-5 Chip nopadding">
                {"SKILL"}
            </div>
            <div class="chip-col-1-5 Chip nopadding">
                {"DMG"}
            </div>
            <div class="chip-col-2 Chip nopadding">
                {"ELEM"}
            </div>
        </div>
    }
}

#[derive(Properties, Clone, PartialEq)]
pub(crate) struct LibraryProps {
    pub active: bool,
}

pub(crate) enum LibraryMessage {
    ChangeSort(ChipSortOptions),
    ChangeFilter(String),
    SetHighlightedChip(String),
    DoNothing,
}

impl From<std::option::NoneError> for LibraryMessage {
    fn from(_: std::option::NoneError) -> Self {
        LibraryMessage::DoNothing
    }
}

impl std::ops::Try for LibraryMessage {
    type Ok = Self;
    type Error = Self;

    fn into_result(self) -> Result<Self::Ok, Self::Error> {
        
        match self {
            LibraryMessage::DoNothing => Err(LibraryMessage::DoNothing),
            _ => Ok(self)
        }
    }
    fn from_error(_: Self::Error) -> Self {
        LibraryMessage::DoNothing
    }
    fn from_ok(v: Self::Ok) -> Self {
        v
    }
    
}

fn handle_mouseover_event(e: MouseEvent) -> LibraryMessage {
    let target = e.current_target()?;

    let div = target.dyn_ref::<web_sys::HtmlElement>()?;

    let id = div.id();

    let name = id.get(2..)?.to_owned();

    //let chip = ChipLibrary::get_instance().library.get(name)?.clone();

    LibraryMessage::SetHighlightedChip(name)
}

pub(crate) struct LibraryComponent{
    props: LibraryProps,
    _link: ComponentLink<Self>,
    sort_by: ChipSortOptions,
    filter_by: String,
    sort_changed: Callback<ChangeData>,
    text_changed: Callback<InputData>,
    chip_mouseover: Callback<MouseEvent>,
    set_desc_bus: Dispatcher<ChipDescMsgBus>,
}

impl Component for LibraryComponent {
    type Message = LibraryMessage;
    type Properties = LibraryProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        let sort_changed = _link.callback(|e: ChangeData| {
            //web_sys::console::log_1(&wasm_bindgen::JsValue::from_str("sort change emitted"));
            if let ChangeData::Select(val) = e {
                LibraryMessage::ChangeSort(ChipSortOptions::from(val.value().as_ref()))
            } else {
                LibraryMessage::DoNothing
            }
        });
        let text_changed = _link.callback(|e: InputData| {
            //web_sys::console::log_1(&wasm_bindgen::JsValue::from_str("text change emitted"));
            LibraryMessage::ChangeFilter(e.value)
        });
        let chip_mouseover = _link.callback(handle_mouseover_event);
        let set_desc_bus = ChipDescMsgBus::dispatcher();
        Self {
            props,
            _link,
            sort_by: ChipSortOptions::Name,
            filter_by: String::default(),
            sort_changed,
            text_changed,
            chip_mouseover,
            set_desc_bus,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            LibraryMessage::ChangeSort(opt) => {
                
                if self.sort_by == opt {
                    return false;
                }
                self.sort_by = opt;
                true
            }
            LibraryMessage::DoNothing => false,
            LibraryMessage::ChangeFilter(val) => {
                self.filter_by = val.trim().to_ascii_lowercase();
                true
            }
            LibraryMessage::SetHighlightedChip(name) => {
                self.set_desc_bus.send(ChipDescMsg::SetDesc(name));
                false
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if props.active == false && self.props.active == true {
            self.props = props;
            return true;
        } else if props.active == true && self.props.active == false {
            self.props = props;
            self.set_desc_bus.send(ChipDescMsg::ClearDesc);
            return true;
        } else {
            return false;
        }
    }

    fn view(&self) -> Html {

        let (col1_display, col2_display, library_containter_class) = if self.props.active {
                ("left-panel nopadding", "middle-panel nopadding", "Folder activeFolder")
            } else {
                ("inactiveTab", "inactiveTab", "Folder")
            };

        html! {
            <>
            <div class=col1_display>
                <ChipSortBox include_owned={false} sort_by={self.sort_by} sort_changed={self.sort_changed.clone()}/>
                {self.build_search_box()}
                <br/>
                {list_spectators()}
            </div>
            <div class=col2_display>
                <div class=library_containter_class>
                    <LibraryTopRow/>
                    {self.build_library_chips()}
                 </div>
            </div>
            </>
        }
    }

}

impl LibraryComponent {

    fn build_search_box(&self) -> Html {
        let text_changed = self.text_changed.clone();

        html! {
            <>
            <br/>
            <span unselectable="on" class="Chip">{"Search"}</span>
            <input type="text" class="chip-search-input" value={&self.filter_by} oninput=text_changed/>
            </>
        }
    }

    fn build_library_chips(&self) -> Html {
        let mut chip_lib = self.fetch_chips();
        if chip_lib.is_empty() {
           return html!{
                <span class="noselect">
                {"Nothing matched your search"}
                </span>
            }
        }

        chip_lib.drain(..).map(|chip|{
            html!{    
                <LibraryChip chip={chip} on_mouse_enter={self.chip_mouseover.clone()}/>
            }
        }).collect::<Html>()
    }

    fn fetch_chips(&self) -> Vec<&Rc<BattleChip>> {
        let mut chip_lib = if self.filter_by.is_empty() {
            ChipLibrary::get_instance().library.values().collect::<Vec<&Rc<BattleChip>>>()
        } else {
            ChipLibrary::get_instance().library.values().filter(|chip| {
                chip.name.to_ascii_lowercase().starts_with(&self.filter_by)
            }).collect::<Vec<&Rc<BattleChip>>>()
        };

        match self.sort_by {
            ChipSortOptions::Name => {
                chip_lib.sort_unstable_by(|a, b| {
                    a.class.cmp(&b.class).then_with(||a.name.cmp(&b.name))
                });
            }
            ChipSortOptions::Element => {
                chip_lib.sort_unstable_by(|a, b| {
                    a.element.cmp(&b.element).then_with(||a.name.cmp(&b.name))
                });
            }
            ChipSortOptions::MaxDamage => {
                chip_lib.sort_unstable_by(|a, b| {
                    unsafe{a.max_dmg().partial_cmp(&b.max_dmg()).unchecked_unwrap()}.reverse().then_with(||a.name.cmp(&b.name))
                });
            }
            ChipSortOptions::AverageDamage => {
                chip_lib.sort_unstable_by(|a, b| {
                    unsafe{a.avg_dmg().partial_cmp(&b.avg_dmg()).unchecked_unwrap()}.reverse().then_with(||a.name.cmp(&b.name))
                });
            }
            ChipSortOptions::Skill => {
                chip_lib.sort_unstable_by(|a, b| {
                    a.skill().cmp(&b.skill()).then_with(||a.name.cmp(&b.name))
                });
            }
            ChipSortOptions::Range => {
                chip_lib.sort_unstable_by(|a, b| {
                    a.range.cmp(&b.range).then_with(||a.name.cmp(&b.name))
                });
            }
            ChipSortOptions::Owned => unsafe{core::hint::unreachable_unchecked()},
        }
        chip_lib
    }
}