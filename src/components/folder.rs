use unchecked_unwrap::UncheckedUnwrap;
use yew::prelude::*;
use crate::components::ChipSortOptions;
use crate::chip_library::{ChipLibrary, FolderChip as FldrChp};
use crate::util::{alert, generate_element_images};


#[derive(Properties, Clone)]
pub struct FolderProps {
    pub active: bool,
    pub set_msg_callback: Callback<String>,
}

pub enum FolderMsg {
    ChangeSort(ChipSortOptions),
    ChangeUsed(usize),
    ReturnToPack(usize),
    JackOut,
    ClearFolder,
    DoNothing,
    ForceRedraw,
}

pub struct FolderComponent {
    props: FolderProps,
    link: ComponentLink<Self>,
    sort_by: ChipSortOptions,
}

impl Component for FolderComponent {
    type Message = FolderMsg;
    type Properties = FolderProps;
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            props,
            link,
            sort_by: ChipSortOptions::Name,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            FolderMsg::ChangeSort(sort_by) => {
                if self.sort_by != sort_by {
                    self.sort_by = sort_by;
                    return true;
                } else {
                    return false
                }
            },

            FolderMsg::ClearFolder => {
                let count = ChipLibrary::get_instance().clear_folder();
                self.props.set_msg_callback.emit(format!("{} chips have been returned to your pack", count));
                true
            },

            FolderMsg::JackOut => {
                let count = ChipLibrary::get_instance().jack_out();
                self.props.set_msg_callback.emit(format!("{} chips have been marked as unused", count));
                true
            },
            FolderMsg::ReturnToPack(idx) => {
                let chip_library = ChipLibrary::get_instance();
                let folder = chip_library.folder.read().unwrap();
                self.props.set_msg_callback.emit(format!("A copy of {} has been returned to your pack",folder.get(idx).unwrap().name));
                drop(folder);
                if let Err(why) = chip_library.return_fldr_chip_to_pack(idx) {
                   unsafe{alert(why)};
                }
                true
            },
            FolderMsg::ChangeUsed(idx) => {
                let chip_library = ChipLibrary::get_instance();
                let mut folder = chip_library.folder.write().unwrap();
                folder[idx].used = !folder[idx].used;
                true
            }
            FolderMsg::DoNothing => false,
            FolderMsg::ForceRedraw => true,
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
        let (folder_containter_class, outer_container_class) = if self.props.active {("container-fluid Folder activeFolder", "container-fluid")} else {("container-fluid Folder", "inactiveTab")};

        html! {
            <div class={outer_container_class}>
                <div class="row nopadding">
                    <div class="col-10 nopadding">
                        <div class={folder_containter_class}>
                                {self.build_top_row()}
                                {self.build_folder()}
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

impl FolderComponent {
    fn build_top_row(&self) -> Html {
        html! {
            <div class="row sticky-top justify-content-center debug" style="background-color: gray">
                <div class="col-1 Chip nopadding debug"/>
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
                    {"USED"}
                </div>
            </div>
        }
    }

    fn build_folder(&self) -> Html {
        let mut folder = ChipLibrary::get_instance().folder.write().unwrap();
        
        match self.sort_by {
            ChipSortOptions::Name => {
                folder.sort_by(|a, b| {
                    a.chip.kind.cmp(&b.chip.kind).then_with(||a.chip.name.cmp(&b.chip.name))
                });
            }
            ChipSortOptions::Element => {
                folder.sort_by(|a, b| {
                    a.chip.element.cmp(&b.chip.element).then_with(||a.chip.name.cmp(&b.chip.name))
                });
            }
            ChipSortOptions::MaxDamage => {
                folder.sort_by(|a, b| {
                    unsafe{a.chip.max_dmg().partial_cmp(&b.chip.max_dmg()).unchecked_unwrap()}.reverse().then_with(||a.chip.name.cmp(&b.chip.name))
                });
            }
            ChipSortOptions::AverageDamage => {
                folder.sort_by(|a, b| {
                    unsafe{a.chip.avg_dmg().partial_cmp(&b.chip.avg_dmg()).unchecked_unwrap()}.reverse().then_with(||a.chip.name.cmp(&b.chip.name))
                });
            }
            ChipSortOptions::Skill => {
                folder.sort_by(|a, b| {
                    a.chip.skill().cmp(&b.chip.skill()).then_with(||a.chip.name.cmp(&b.chip.name))
                });
            }
            ChipSortOptions::Range => {
                folder.sort_by(|a, b| {
                    a.chip.range.cmp(&b.chip.range).then_with(||a.chip.name.cmp(&b.chip.name))
                });
            }
            ChipSortOptions::Owned => unsafe{core::hint::unreachable_unchecked()}
        }

        self.fldr_to_html(&folder)

        /*
        let folder_len = folder.len();

        //drop lock, no longer needed
        drop(folder);
        let return_to_pack = self.link.callback(|idx: usize| FolderMsg::ReturnToPack(idx));
        
            
        (0..folder_len).map(|idx|{
            html!{
                <FolderChip index={idx} set_msg_callback={self.props.set_msg_callback.clone()} return_to_pack_callback={return_to_pack.clone()}/>
            }
        }).collect::<Html>()
        */
    }

    fn fldr_to_html(&self, chips: &[FldrChp]) -> Html {
        let return_to_pack = self.link.callback(|idx: usize| FolderMsg::ReturnToPack(idx));

        chips.iter().zip(0..chips.len()).map(|(chip, idx)| {
            let chip_css = if chip.used {
                "UsedChip"
            } else {
                chip.chip.kind.to_css_class()
            };
            let return_clone = return_to_pack.clone();
            let index_clone = idx;
            let on_dbl_click = Callback::once(move |_:MouseEvent| return_clone.emit(idx));
            html!{
                <div class=("row justify-content-center noselect chipHover", chip_css) ondoubleclick={on_dbl_click} id={format!("F1_{}", idx)}>
                    <div class="col-1 nopadding debug">
                        {idx + 1}
                    </div>
                    <div class="col-3 nopadding debug" style="white-space: nowrap">
                        {&chip.chip.name}
                    </div>
                    <div class="col-2 nopadding debug">
                        {chip.chip.skill()}
                    </div>
                    <div class="col-1 nopadding debug">
                        {&chip.chip.damage}
                    </div>
                    <div class="col-1 nopadding debug centercontent">
                        {&chip.chip.range}
                    </div>
                    <div class="col-1 nopadding debug centercontent" style="white-space: nowrap">
                        {&chip.chip.hits}
                    </div>
                    <div class="col-1 nopadding debug centercontent">
                        {generate_element_images(&chip.chip.element)}
                    </div>
                    <div class="col-1 nopadding centercontent" ondoubleclick={self.link.callback(|e:MouseEvent| {e.stop_propagation(); FolderMsg::DoNothing})}>
                        <input name="chipUsed" type="checkbox" checked={chip.used} onchange={self.link.callback(move |_| FolderMsg::ChangeUsed(index_clone))}/>
                    </div>
                </div>
            }

        }).collect::<Html>()
    }

    fn build_sort_box(&self) -> Html {
        let sort_change_callback = self.link.callback(|e: ChangeData| {
            if let ChangeData::Select(val) = e {
                FolderMsg::ChangeSort(ChipSortOptions::from(val.value().as_ref()))
            } else {
                FolderMsg::DoNothing
            }
        });

        html!{
            <>
            <span unselectable="on" class="Chip">{"Sort By"}</span>
            <select value={&self.sort_by} style="width: 100%" class="custom-select" onchange={sort_change_callback}>
                <option value="Name">{"Name"}</option>
                <option value="Element">{"Element"}</option>
                <option value="MaxDamage">{"MaxDamage"}</option>
                <option value="AverageDamage">{"AverageDamage"}</option>
                <option value="Skill">{"Skill"}</option>
                <option value="Range">{"Range"}</option>
            </select>
            </>
        }
    }
}

