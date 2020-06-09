use yew::prelude::*;
use yew::html::{ChangeData, InputData};
use std::sync::Arc;
use unchecked_unwrap::UncheckedUnwrap;

use crate::components::{ChipSortOptions, library_chip::LibraryChip, sort_box::ChipSortBox};
use crate::chip_library::ChipLibrary;
use crate::chip_library::battle_chip::BattleChip;



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
    }
}

#[derive(Properties, Clone)]
pub struct LibraryProps {
    pub active: bool,
}

pub enum LibraryMessage {
    ChangeSort(ChipSortOptions),
    ChangeFilter(String),
    DoNothing,
}

pub struct LibraryComponent{
    props: LibraryProps,
    link: ComponentLink<Self>,
    sort_by: ChipSortOptions,
    filter_by: String,
    sort_changed: Callback<ChangeData>,
    text_changed: Callback<InputData>,
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
        Self {
            props,
            link,
            sort_by: ChipSortOptions::Name,
            filter_by: String::default(),
            sort_changed,
            text_changed,
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
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props.active == props.active {
            self.props = props;
            false
        } else {
            self.props = props;
            true
        }
    }

    fn view(&self) -> Html {

        let (library_containter_class, outer_container_class) = if self.props.active {("container-fluid Folder activeFolder", "container-fluid")} else {("container-fluid Folder", "inactiveTab")};
        
        html! {
            <div class={outer_container_class}>
                <div class="row nopadding">
                    <div class="col-10 nopadding">
                        <div class={library_containter_class}>
                                <LibraryTopRow/>
                                {self.build_library_chips()}
                        </div>
                    </div>
                    <div class="col-2 nopadding">
                        <ChipSortBox include_owned={false} sort_by={self.sort_by} sort_changed={self.sort_changed.clone()}/>
                        {self.build_search_box()}
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
                <LibraryChip chip={chip}/>
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

}