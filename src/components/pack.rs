use crate::chip_library::{ChipLibrary, PackChip};
use crate::components::{ChipSortOptions, pack_chip::PackChipComponent, sort_box::ChipSortBox};
use yew::prelude::*;
use yew::agent::{Dispatcher, Dispatched};
use crate::agents::global_msg::{GlobalMsgBus, Request as GlobalMsgReq};
use crate::util::alert;
use yew::events::MouseEvent;
use wasm_bindgen::JsCast;

use std::collections::HashMap;
use unchecked_unwrap::UncheckedUnwrap;

#[derive(Properties, Clone)]
pub struct PackProps {
    pub active: bool,
}

pub enum PackMsg {
    ChangeSort(ChipSortOptions),
    MoveToFolder(String),
    JackOut,
    ExportJson,
    ExportTxt,
    EraseData,
    ImportJson,
    DoNothing,
}

impl From<std::option::NoneError> for PackMsg {
    fn from(_: std::option::NoneError) -> Self {
        PackMsg::DoNothing
    }
}

impl std::ops::Try for PackMsg {
    type Ok = Self;
    type Error = Self;

    fn into_result(self) -> Result<Self::Ok, Self::Error> {
        
        match self {
            PackMsg::DoNothing => Err(PackMsg::DoNothing),
            _ => Ok(self)
        }
    }
    fn from_error(v: Self::Error) -> Self {
        PackMsg::DoNothing
    }
    fn from_ok(v: Self::Ok) -> Self {
        v
    }
    
}

pub struct PackComponent {
    props: PackProps,
    sort_by: ChipSortOptions,
    link: ComponentLink<Self>,
    event_bus: Dispatcher<GlobalMsgBus>,
    sort_changed: Callback<ChangeData>,
    move_to_folder_callback: Callback<MouseEvent>
}

fn move_to_folder_callback(e: MouseEvent) -> PackMsg {
    
    let target = e.current_target()?;

    let div = target.dyn_ref::<web_sys::HtmlElement>()?;

    let id = div.id();
    let val = id[2..].to_owned();
    web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(&format!("{} is being added to folder", val)));
    PackMsg::MoveToFolder(val)
}

impl Component for PackComponent {
    type Message = PackMsg;
    type Properties = PackProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let event_bus = GlobalMsgBus::dispatcher();
        let move_to_folder_callback = link.callback(move_to_folder_callback);
        let sort_changed = link.callback(|e: ChangeData| {
            //web_sys::console::log_1(&wasm_bindgen::JsValue::from_str("sort change emitted"));
            if let ChangeData::Select(val) = e {
                PackMsg::ChangeSort(ChipSortOptions::from(val.value().as_ref()))
            } else {
                PackMsg::DoNothing
            }
        });
        Self {
            props,
            link,
            sort_by: ChipSortOptions::Name,
            event_bus,
            move_to_folder_callback,
            sort_changed,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            PackMsg::ChangeSort(sort_opt) => {
                if self.sort_by != sort_opt {
                    self.sort_by = sort_opt;
                    return true;
                }
                false
            },
            PackMsg::JackOut => {
                let count = ChipLibrary::get_instance().jack_out();
                self.event_bus.send(GlobalMsgReq::SetHeaderMsg(format!("{} chips have been marked as unused", count)));
                true
            },
            PackMsg::ExportJson => {todo!()},
            PackMsg::ExportTxt => {todo!()},
            PackMsg::EraseData => {todo!()}
            PackMsg::ImportJson => {todo!()},
            PackMsg::DoNothing => false,
            PackMsg::MoveToFolder(name) => self.move_chip_to_folder(&name)
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
        
        let (pack_containter_class, outer_container_class) = if self.props.active {("container-fluid Folder activeFolder", "container-fluid")} else {("container-fluid Folder", "inactiveTab")};
        html! {
            <div class={outer_container_class}>
                <div class="row nopadding">
                    <div class="col-10 nopadding">
                        <div class={pack_containter_class}>
                            <PackTopRow />
                            {self.build_pack_chips()}
                        </div>
                    </div>
                    <div class="col-2 nopadding">
                        <ChipSortBox include_owned={true} sort_by={self.sort_by} sort_changed={self.sort_changed.clone()}/>
                        <br/>
                        <br/>
                        <br/>
                        {self.generate_buttons()}
                    </div>
                </div>
            </div>
        }
    }
}

impl PackComponent {

    fn build_pack_chips(&self) -> Html {
        let lib = ChipLibrary::get_instance();
        let pack = lib.pack.read().unwrap();
        if pack.len() == 0 {
           return html!{ 
                <span class="noselect Chip">
                {"Your pack is empty!"}
                </span>
           }
        }

        let mut pack_list = self.fetch_and_sort_pack(&pack);
        
        pack_list.drain(..).map(|chip| {
            html!{
                    <PackChipComponent used={chip.used} owned={chip.owned} chip={chip.chip.clone()} add_to_folder={self.move_to_folder_callback.clone()}/>
                }
        }).collect::<Html>()

    }

    fn fetch_and_sort_pack<'a>(&self, pack: &'a HashMap<String, PackChip>) -> Vec<&'a PackChip> {
        let mut pack_list = pack.values().collect::<Vec<&PackChip>>();
        match self.sort_by {
            ChipSortOptions::Name => {
                pack_list.sort_unstable_by(|a, b| {
                    a.chip.kind.cmp(&b.chip.kind).then_with(||a.chip.name.cmp(&b.chip.name))
                });
            }
            ChipSortOptions::Element => {
                pack_list.sort_unstable_by(|a, b| {
                    a.chip.element.cmp(&b.chip.element).then_with(||a.chip.name.cmp(&b.chip.name))
                });
            }
            ChipSortOptions::MaxDamage => {
                pack_list.sort_unstable_by(|a, b| {
                    unsafe{a.chip.max_dmg().partial_cmp(&b.chip.max_dmg()).unchecked_unwrap()}.reverse().then_with(||a.chip.name.cmp(&b.chip.name))
                });
            }
            ChipSortOptions::AverageDamage => {
                pack_list.sort_unstable_by(|a, b| {
                    unsafe{a.chip.avg_dmg().partial_cmp(&b.chip.avg_dmg()).unchecked_unwrap()}.reverse().then_with(||a.chip.name.cmp(&b.chip.name))
                });
            }
            ChipSortOptions::Skill => {
                pack_list.sort_unstable_by(|a, b| {
                    a.chip.skill().cmp(&b.chip.skill()).then_with(||a.chip.name.cmp(&b.chip.name))
                });
            }
            ChipSortOptions::Range => {
                pack_list.sort_unstable_by(|a, b| {
                    a.chip.range.cmp(&b.chip.range).then_with(||a.chip.name.cmp(&b.chip.name))
                });
            }
            ChipSortOptions::Owned => {
                pack_list.sort_unstable_by(|a,b| {
                    a.owned.cmp(&b.owned).reverse().then_with(|| a.chip.name.cmp(&b.chip.name))
                });
            }
        }
        pack_list
    }

    fn generate_buttons(&self) -> Html {
        let jack_out_callback = self.link.callback(|_: MouseEvent| PackMsg::JackOut);
        let export_json_callback = self.link.callback(|_: MouseEvent| PackMsg::ExportJson);
        let export_txt_callback = self.link.callback(|_: MouseEvent| PackMsg::ExportTxt);
        let erase_data_callback = self.link.callback(|_: MouseEvent| PackMsg::EraseData);
        let import_data_callback = self.link.callback(|_: MouseEvent| PackMsg::ImportJson);

        html!{
            <div class="centercontent">
                <button class="btn sideButtons ripple" onclick=jack_out_callback>
                    <span class="Chip">{"Jack Out"}</span>
                </button>
                <br/>
                <button class="btn sideButtons ripple" onclick=export_json_callback>
                    <span class="Chip">{"Export JSON"}</span>
                </button>
                <br/>
                <button class="btn sideButtons ripple" onclick=export_txt_callback>
                    <span class="Chip">{"Export Txt"}</span>
                </button>
                <br/>
                <button class="btn sideButtons ripple" onclick=erase_data_callback>
                    <span class="Chip">{"Erase Data"}</span>
                </button>
                <br/>
                <button class="btn sideButtons ripple" onclick=import_data_callback>
                    <span class="Chip">{"Import Data"}</span>
                </button>
            </div>
        }

    }

    fn move_chip_to_folder(&mut self, name: &str) -> bool {
        match ChipLibrary::get_instance().move_to_folder(&name) {
            Ok(_) => {
                let msg = format!("A copy of {} has been added to your folder", name);
                self.event_bus.send(GlobalMsgReq::SetHeaderMsg(msg));
            }
            Err(msg) => {
             unsafe{alert(msg)};
            }
        }
        true
    }
}


pub struct PackTopRow;

impl Component for PackTopRow {
    
    type Message = ();
    type Properties = ();
    
    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self{}
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
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
                <div class="col-1 Chip nopadding debug">
                    {"OWN"}
                </div>
                <div class="col-1 Chip nopadding debug">
                    {"USED"}
                </div>
            </div>
        }
    }

}