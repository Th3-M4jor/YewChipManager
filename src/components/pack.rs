use crate::chip_library::{ChipLibrary, PackChip};
use crate::components::{ChipSortOptions, pack_chip::PackChip as PackChipComponent};
use yew::prelude::*;

use std::collections::HashMap;

#[derive(Properties, Clone)]
pub struct PackProps {
    pub active: bool,
    pub set_msg_callback: Callback<String>,
}

pub enum PackMsg {
    ChangeSort(ChipSortOptions),
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
}

impl Component for PackComponent {
    type Message = PackMsg;
    type Properties = PackProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            props,
            link,
            sort_by: ChipSortOptions::Name,
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
                self.props.set_msg_callback.emit(format!("{} chips have been marked as unused", count));
                true
            },
            PackMsg::ExportJson => {todo!()},
            PackMsg::ExportTxt => {todo!()},
            PackMsg::EraseData => {todo!()}
            PackMsg::ImportJson => {todo!()},
            PackMsg::DoNothing => false,
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
                    </div>
                </div>
            </div>
        }
    }
}

impl PackComponent {
    fn build_top_row(&self) -> Html {
        html! {
            <div class="row sticky-top justify-content-center debug" style="background-color: gray">
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
                <>
                {"Your pack is empty!"}
                </>
           }
        }

        let pack_list = self.fetch_and_sort_pack(&pack);

        html!{
            <>
            {
                pack_list.iter().map(|chip| {
                    html!{
                        <PackChipComponent name={&chip.chip.name} set_msg_callback={self.props.set_msg_callback.clone()}/>
                    }
                }).collect::<Html>()
            }
            </>
        }

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
                    a.chip.max_dmg().partial_cmp(&b.chip.max_dmg()).unwrap().reverse().then_with(||a.chip.name.cmp(&b.chip.name))
                });
            }
            ChipSortOptions::AverageDamage => {
                pack_list.sort_unstable_by(|a, b| {
                    a.chip.avg_dmg().partial_cmp(&b.chip.avg_dmg()).unwrap().reverse().then_with(||a.chip.name.cmp(&b.chip.name))
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

}

