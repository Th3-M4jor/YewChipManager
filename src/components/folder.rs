use yew::prelude::*;
use crate::components::{ChipSortOptions, folder_chip::FolderChip};
use crate::chip_library::ChipLibrary;

#[derive(Properties, Clone)]
pub struct FolderProps {
    pub active: bool,
    pub set_msg_callback: Callback<String>,
}

pub enum FolderMsg {
    ChangeSort(ChipSortOptions),
    JackOut,
    ClearFolder,
    DoNothing,
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

            FolderMsg::DoNothing => {false},
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
                    a.chip.max_dmg().partial_cmp(&b.chip.max_dmg()).unwrap().reverse().then_with(||a.chip.name.cmp(&b.chip.name))
                });
            }
            ChipSortOptions::AverageDamage => {
                folder.sort_by(|a, b| {
                    a.chip.avg_dmg().partial_cmp(&b.chip.avg_dmg()).unwrap().reverse().then_with(||a.chip.name.cmp(&b.chip.name))
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
            ChipSortOptions::Owned => {unreachable!()}
        }

        let folder_len = folder.len();

        //drop lock, no longer needed
        drop(folder);

        html! {
            <>
            {(0..folder_len).map(|idx|{
                html!{
                    <FolderChip index={idx} set_msg_callback={self.props.set_msg_callback.clone()}/>
                }
            }).collect::<Html>()}
            </>
        }

    }

}

