use yew::prelude::*;
use yew::html::{ChangeData, InputData};
use std::sync::Arc;
use unchecked_unwrap::UncheckedUnwrap;
use wasm_bindgen::JsCast;

use crate::components::{ChipSortOptions, library_chip::LibraryChip, sort_box::ChipSortBox};
use crate::chip_library::{BattleChip, ChipLibrary};



pub struct LibraryTopRow;

impl Component for LibraryTopRow {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self{}
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        /*
        html! {
            <div class="row sticky-top justify-content-center debug noselect" style="background-color: gray">
                <div class="col-3 Chip nopadding debug" style="white-space: nowrap">
                    {"NAME"}
                </div>
                <div class="col-2 Chip nopadding debug">
                    {"SKILL"}
                </div>
                <div class="col-1 Chip nopadding debug">
                    {"DMG"}
                </div>
                <div class="col-1 Chip nopadding debug">
                    {"RANGE"}
                </div>
                <div class="col-1 Chip nopadding debug">
                    {"HITS"}
                </div>
                <div class="col-1 Chip nopadding debug"/>
            </div>
        }
        */
        html! {
            <div class="row sticky-top justify-content-center noselect" style="background-color: gray">
                <div class="col-4 Chip nopadding" style="white-space: nowrap">
                    {"NAME"}
                </div>
                <div class="col-3 Chip nopadding">
                    {"SKILL"}
                </div>
                <div class="col-2 Chip nopadding"/>
            </div>
        }
    }
}

#[derive(Properties, Clone, PartialEq)]
pub struct LibraryProps {
    pub active: bool,
}

pub enum LibraryMessage {
    ChangeSort(ChipSortOptions),
    ChangeFilter(String),
    SetHighlightedChip(Arc<BattleChip>),
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
    fn from_error(v: Self::Error) -> Self {
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

    let name = &id[2..];

    let chip = ChipLibrary::get_instance().library.get(name)?.clone();

    LibraryMessage::SetHighlightedChip(chip)
}

pub struct LibraryComponent{
    props: LibraryProps,
    link: ComponentLink<Self>,
    sort_by: ChipSortOptions,
    filter_by: String,
    sort_changed: Callback<ChangeData>,
    text_changed: Callback<InputData>,
    highlighted_chip: Option<Arc<BattleChip>>,
    chip_mouseover: Callback<MouseEvent>,
    chip_anim_count: u32,
}

impl Component for LibraryComponent {
    type Message = LibraryMessage;
    type Properties = LibraryProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let sort_changed = link.callback(|e: ChangeData| {
            //web_sys::console::log_1(&wasm_bindgen::JsValue::from_str("sort change emitted"));
            if let ChangeData::Select(val) = e {
                LibraryMessage::ChangeSort(ChipSortOptions::from(val.value().as_ref()))
            } else {
                LibraryMessage::DoNothing
            }
        });
        let text_changed = link.callback(|e: InputData| {
            //web_sys::console::log_1(&wasm_bindgen::JsValue::from_str("text change emitted"));
            LibraryMessage::ChangeFilter(e.value)
        });
        let chip_mouseover = link.callback(handle_mouseover_event);
        Self {
            props,
            link,
            sort_by: ChipSortOptions::Name,
            filter_by: String::default(),
            sort_changed,
            text_changed,
            highlighted_chip: None,
            chip_mouseover,
            chip_anim_count: 0,
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
            LibraryMessage::SetHighlightedChip(chip) => {
                if let Some(curr_chip) = &self.highlighted_chip {
                    if Arc::ptr_eq(curr_chip, &chip) {
                        return false;
                    }
                }
                self.highlighted_chip = Some(chip);
                self.chip_anim_count += 1;
                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if props.active == false && self.props.active == true {
            self.props = props;
            return true;
        } else if props.active == true && self.props.active == false {
            self.props = props;
            self.highlighted_chip.take();
            return true;
        } else {
            return false;
        }
    }

    fn view(&self) -> Html {

        let (library_containter_class, outer_container_class) = if self.props.active {("container-fluid Folder activeFolder", "container-fluid")} else {("container-fluid Folder", "inactiveTab")};
        
        html! {
            <div class={outer_container_class}>
                <div class="row nopadding">
                    <div class="col-2 nopadding">
                        <ChipSortBox include_owned={false} sort_by={self.sort_by} sort_changed={self.sort_changed.clone()}/>
                        {self.build_search_box()}
                    </div>
                    <div class="col-7 nopadding">
                        <div class={library_containter_class}>
                                <LibraryTopRow/>
                                {self.build_library_chips()}
                        </div>
                    </div>
                    <div class="col-3 nopadding chipDescBackground">
                        {self.highlighted_chip_text()}
                    </div>
                </div>
            </div>
        }
    }

}

impl LibraryComponent {

    fn build_search_box(&self) -> Html {
        let text_changed = self.text_changed.clone();

        html! {
            <>
            <br/>
            <br/>
            <span unselectable="on" class="Chip">{"Search"}</span>
            <input type="text" class="form-control form-control-sm" value={&self.filter_by} oninput={text_changed}/>
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

    fn fetch_chips(&self) -> Vec<&Arc<BattleChip>> {
        let mut chip_lib = if self.filter_by.is_empty() {
            ChipLibrary::get_instance().library.values().collect::<Vec<&Arc<BattleChip>>>()
        } else {
            ChipLibrary::get_instance().library.values().filter(|chip| {
                chip.name.to_ascii_lowercase().starts_with(&self.filter_by)
            }).collect::<Vec<&Arc<BattleChip>>>()
        };

        match self.sort_by {
            ChipSortOptions::Name => {
                chip_lib.sort_unstable_by(|a, b| {
                    a.kind.cmp(&b.kind).then_with(||a.name.cmp(&b.name))
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

    fn highlighted_chip_text(&self) -> Html {
        if let Some(chip) = &self.highlighted_chip {
            let chip_anim_class = if self.chip_anim_count % 2 == 0 {
                "chipWindowOne"
            } else {
                "chipWindowTwo"
            };
            let font_style = if chip.description.len() > 450 {
                "font-size: 12px; text-align: left"
            } else if chip.description.len() > 300 {
                "font-size: 14px; text-align: left"
            } else {
                "font-size: 15px; text-align: left"
            };
            html!{
                <div class={format!("{} {}",chip_anim_class, chip.kind.to_css_class())} style="padding: 3px">
                    {build_damage_span(chip)}
                    {build_range_span(chip)}
                    {build_hits_span(chip)}
                    <br/>
                    <div style={font_style}>
                        {&chip.description}
                    </div>
                </div>
            }
        } else {
            html!{}
        }
    }
}

#[inline]
fn build_damage_span(chip: &BattleChip) -> Html {
    if chip.damage == "--" {
        html!{}
    } else {
        html!{
            <span style="float: left">{&chip.damage}</span>
        }
    }
}

#[inline]
fn build_range_span(chip: &BattleChip) -> Html {
    html!{
        <span>{&chip.range}</span>
    }
}

fn build_hits_span(chip: &BattleChip) -> Html {
    if chip.hits == "0" {
        html!{}
    } else {
        html!{
            <span style="float: right">{format!("{} hits", chip.hits)}</span>
        }
    }
}