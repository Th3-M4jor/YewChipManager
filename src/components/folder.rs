use unchecked_unwrap::UncheckedUnwrap;
use yew::prelude::*;
use crate::components::{ChipSortOptions, folder_chip::FolderChipComponent as FolderChip, sort_box::ChipSortBox};
use crate::chip_library::{ChipLibrary, FolderChip as FldrChp};
use crate::util::{alert, generate_element_images};
use web_sys::MouseEvent;
use wasm_bindgen::JsCast;

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

impl From<std::option::NoneError> for FolderMsg {
    fn from(_: std::option::NoneError) -> Self {
        FolderMsg::DoNothing
    }
}

impl std::ops::Try for FolderMsg {
    type Ok = Self;
    type Error = Self;

    fn into_result(self) -> Result<Self::Ok, Self::Error> {
        
        match self {
            FolderMsg::DoNothing => Err(FolderMsg::DoNothing),
            _ => Ok(self)
        }
    }
    fn from_error(v: Self::Error) -> Self {
        FolderMsg::DoNothing
    }
    fn from_ok(v: Self::Ok) -> Self {
        v
    }
    
}

pub struct FolderComponent {
    props: FolderProps,
    link: ComponentLink<Self>,
    sort_by: ChipSortOptions,
    return_to_pack: Callback<MouseEvent>,
    change_used_callback: Callback<MouseEvent>,
    sort_change_callback: Callback<ChangeData>,
}

fn return_pack_callback(e: MouseEvent) -> FolderMsg {
    
    let target = e.current_target()?;
    let div = target.dyn_ref::<web_sys::HtmlElement>()?;
    let id = div.id();
    let val = id[3..].parse::<usize>().ok()?;
    FolderMsg::ReturnToPack(val)
}

fn change_used_callback_fn(e: MouseEvent) -> FolderMsg {
    
    let target = e.current_target()?;
    let div = target.dyn_ref::<web_sys::HtmlElement>()?;
    let id = div.id();
    let val = id[3..].parse::<usize>().ok()?;
    FolderMsg::ChangeUsed(val)
}

impl Component for FolderComponent {
    type Message = FolderMsg;
    type Properties = FolderProps;
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        //let return_to_pack = link.callback(|idx: usize| FolderMsg::ReturnToPack(idx));
        let change_used_callback = link.callback(change_used_callback_fn);
        let return_to_pack = link.callback(return_pack_callback);
        let sort_change_callback = link.callback(|e: ChangeData| {
            if let ChangeData::Select(val) = e {
                FolderMsg::ChangeSort(ChipSortOptions::from(val.value().as_ref()))
            } else {
                FolderMsg::DoNothing
            }
        });
        Self {
            props,
            link,
            sort_by: ChipSortOptions::Name,
            return_to_pack,
            change_used_callback,
            sort_change_callback,
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
                            <FolderTopRow />
                            {self.build_folder()}
                        </div>
                    </div>
                    <div class="col-2 nopadding">
                    <ChipSortBox sort_by={self.sort_by} include_owned={false} sort_changed={self.sort_change_callback.clone()}/>
                    </div>
                </div>
            </div>
        }

    }
    
}

impl FolderComponent {

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

        //self.fldr_to_html(&folder)

        
        let folder_len = folder.len();

        folder.iter().zip(0..folder_len).map(|(chip, index)|{
            let battlechip = chip.chip.clone();
            html!{
                <FolderChip used={chip.used} idx={index} swap_used={self.change_used_callback.clone()} return_to_pack_callback={self.return_to_pack.clone()} chip={battlechip}/>
            }
        }).collect::<Html>()
        
    }

}

struct FolderTopRow;

impl Component for FolderTopRow {
    type Properties = ();
    type Message = ();

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
}

