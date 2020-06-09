use crate::chip_library::{ChipLibrary, PackChip};
use crate::components::ChipSortOptions;
use yew::prelude::*;
use yew::agent::{Dispatcher, Dispatched};
use crate::agents::global_msg::{GlobalMsgBus, Request as GlobalMsgReq};
use crate::util::{alert, generate_element_images};
use crate::components::pack_chip::PackChipComponent;
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

pub struct PackComponent {
    props: PackProps,
    sort_by: ChipSortOptions,
    link: ComponentLink<Self>,
    event_bus: Dispatcher<GlobalMsgBus>,
    move_to_folder_callback: Callback<MouseEvent>
}

fn move_to_folder_callback(e: MouseEvent) -> PackMsg {
    if let Some(target) = e.current_target() {
        return target.dyn_ref::<web_sys::HtmlElement>().map(|div| {
            let id: String = div.id();
            let val = id[2..].to_owned();
            PackMsg::MoveToFolder(val)
        }).unwrap_or(PackMsg::DoNothing)
    } else {
        return PackMsg::DoNothing;
    }
}

impl Component for PackComponent {
    type Message = PackMsg;
    type Properties = PackProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let event_bus = GlobalMsgBus::dispatcher();
        let move_to_folder_callback = link.callback(move_to_folder_callback);
        Self {
            props,
            link,
            sort_by: ChipSortOptions::Name,
            event_bus,
            move_to_folder_callback,
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
            PackMsg::MoveToFolder(name) => {
               match ChipLibrary::get_instance().move_to_folder(&name) {
                   Ok(_) => {
                       self.event_bus.send(
                        GlobalMsgReq::SetHeaderMsg(
                            format!(
                            "A copy of {} has been added to your folder",
                       name
                    )));
                       true
                   }
                   Err(msg) => {
                    unsafe{alert(msg)};
                    true
                   }
               }
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
        
        let (pack_containter_class, outer_container_class) = if self.props.active {("container-fluid Folder activeFolder", "container-fluid")} else {("container-fluid Folder", "inactiveTab")};
        html! {
            <div class={outer_container_class}>
                <div class="row nopadding">
                    <div class="col-10 nopadding">
                        <div class={pack_containter_class}>
                                {self.build_top_row()}
                                {self.build_pack_chips()}
                        </div>
                    </div>
                    <div class="col-2 nopadding">
                        {self.build_sort_box()}
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
    fn build_top_row(&self) -> Html {
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

        let pack_list = self.fetch_and_sort_pack(&pack);
        
        pack_list.iter().map(|chip| {
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

    fn build_sort_box(&self) -> Html {
        let select_changed = self.link.callback(|e: ChangeData| {
            //web_sys::console::log_1(&wasm_bindgen::JsValue::from_str("sort change emitted"));
            if let ChangeData::Select(val) = e {
                PackMsg::ChangeSort(ChipSortOptions::from(val.value().as_ref()))
            } else {
                PackMsg::DoNothing
            }
        });

        html!{
            <>
            <span unselectable="on" class="Chip">{"Sort By"}</span>
            <select value={&self.sort_by} style="width: 100%" class="custom-select" onchange={select_changed}>
                <option value="Name">{"Name"}</option>
                <option value="Element">{"Element"}</option>
                <option value="MaxDamage">{"MaxDamage"}</option>
                <option value="AverageDamage">{"AverageDamage"}</option>
                <option value="Skill">{"Skill"}</option>
                <option value="Range">{"Range"}</option>
                <option value="Owned">{"Owned"}</option>
            </select>
            </>
        }
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

}

