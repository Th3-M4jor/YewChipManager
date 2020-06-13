use unchecked_unwrap::UncheckedUnwrap;
use yew::{prelude::*, agent::{Dispatcher, Dispatched}};
use crate::{
    components::{
        ChipSortOptions,
        chips::FolderChipComponent as FolderChip,
        sort_box::ChipSortBox
    }, 
    chip_library::ChipLibrary,
    agents::{
        global_msg::{
            GlobalMsgBus,
            Request as GlobalMsgReq
        }, 
        chip_desc::{
            ChipDescMsg,
            ChipDescMsgBus
    }},
    util::alert
};
use web_sys::MouseEvent;
use wasm_bindgen::JsCast;
use std::rc::Rc;
#[derive(Properties, Clone)]
pub struct FolderProps {
    pub active: bool,
}

pub enum FolderMsg {
    ChangeSort(ChipSortOptions),
    ChangeUsed(usize),
    ReturnToPack(usize),
    SetHighlightedChip(usize),
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
    fn from_error(_: Self::Error) -> Self {
        FolderMsg::DoNothing
    }
    fn from_ok(v: Self::Ok) -> Self {
        v
    }
    
}

pub struct FolderComponent {
    props: FolderProps,
    _link: ComponentLink<Self>,
    sort_by: ChipSortOptions,
    return_to_pack: Callback<MouseEvent>,
    change_used_callback: Callback<MouseEvent>,
    chip_mouseover: Callback<MouseEvent>,
    sort_change_callback: Callback<ChangeData>,
    event_bus: Dispatcher<GlobalMsgBus>,
    set_desc_bus: Dispatcher<ChipDescMsgBus>,
}

fn return_pack_callback(e: MouseEvent) -> FolderMsg {
    let target = e.current_target()?;
    let div = target.dyn_ref::<web_sys::HtmlElement>()?;
    let id: String = div.id();
    let index = id[3..].parse::<usize>().ok()?;
    FolderMsg::ReturnToPack(index)
}

fn change_used_callback_fn(e: MouseEvent) -> FolderMsg {
    let target = e.current_target()?;
    let div = target.dyn_ref::<web_sys::HtmlElement>()?;
    let id: String = div.id();
    let index = id[3..].parse::<usize>().ok()?;
    FolderMsg::ChangeUsed(index)
}

fn handle_mouseover_event(e: MouseEvent) -> FolderMsg {
    let target = e.current_target()?;
    let div = target.dyn_ref::<web_sys::HtmlElement>()?;
    let id: String = div.id();
    let index = id[3..].parse::<usize>().ok()?;
    FolderMsg::SetHighlightedChip(index)
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
        let chip_mouseover = link.callback(handle_mouseover_event);
        let set_desc_bus = ChipDescMsgBus::dispatcher();
        let event_bus = GlobalMsgBus::dispatcher();

        Self {
            props,
            _link: link,
            sort_by: ChipSortOptions::Name,
            return_to_pack,
            change_used_callback,
            sort_change_callback,
            event_bus,
            set_desc_bus,
            chip_mouseover,
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
                let msg = format!("{} chips have been returned to your pack", count);
                self.event_bus.send(GlobalMsgReq::SetHeaderMsg(msg));
                self.set_desc_bus.send(ChipDescMsg::ClearDesc);
                true
            },

            FolderMsg::JackOut => {
                let count = ChipLibrary::get_instance().jack_out();
                let msg = format!("{} chips have been marked as unused", count);
                self.event_bus.send(GlobalMsgReq::SetHeaderMsg(msg));
                true
            },
            FolderMsg::ReturnToPack(idx) => {
                let chip_library = ChipLibrary::get_instance();
                let folder = chip_library.folder.borrow_mut();
                let msg = format!("A copy of {} has been returned to your pack", folder.get(idx).unwrap().name);
                self.event_bus.send(GlobalMsgReq::SetHeaderMsg(msg));
                drop(folder);
                if let Err(why) = chip_library.return_fldr_chip_to_pack(idx) {
                   unsafe{alert(why)};
                }
                self.set_desc_bus.send(ChipDescMsg::ClearDesc);
                true
            },
            FolderMsg::ChangeUsed(idx) => {
                let chip_library = ChipLibrary::get_instance();
                let mut folder = chip_library.folder.borrow_mut();
                folder[idx].used = !folder[idx].used;
                true
            },
            FolderMsg::SetHighlightedChip(idx) => {
                let chip_library = ChipLibrary::get_instance();
                let folder = chip_library.folder.borrow();
                let name = folder[idx].name.clone();
                self.set_desc_bus.send(ChipDescMsg::SetDesc(name));
                false
            }
            FolderMsg::DoNothing => false,
            FolderMsg::ForceRedraw => true,
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        // one being set to active has the job of clearing the description text
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
        //let (folder_containter_class, outer_container_class) = if self.props.active {("container-fluid Folder activeFolder", "container-fluid")} else {("container-fluid Folder", "inactiveTab")};

        let (col1_display, col2_display, folder_containter_class) = if self.props.active {
            ("col-2 nopadding", "col-7 nopadding", "container-fluid Folder activeFolder")
        } else {
            ("inactiveTab", "inactiveTab", "container-fluid Folder")
        };

        html!{
            <>
            <div class={col1_display}>
                <ChipSortBox sort_by={self.sort_by} include_owned={false} sort_changed={self.sort_change_callback.clone()}/>
            </div>
            <div class={col2_display}>
                <div class={folder_containter_class}>
                    <FolderTopRow />
                    {self.build_folder()}
                </div>
            </div>
            </>
        }

    }
    
}

impl FolderComponent {

    fn build_folder(&self) -> Html {
        let mut folder = ChipLibrary::get_instance().folder.borrow_mut();
        
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
            ChipSortOptions::Owned => {
                #[cfg(not(debug_assertions))]
                unsafe{core::hint::unreachable_unchecked()};
                #[cfg(debug_assertions)]
                unreachable!();
            },
        }

        
        let folder_len = folder.len();

        folder.iter().zip(0..folder_len).map(|(chip, index)|{
            let battlechip = Rc::clone(&chip.chip);
            html!{
                <FolderChip 
                    used={chip.used} 
                    idx={index} 
                    swap_used={self.change_used_callback.clone()} 
                    return_to_pack_callback={self.return_to_pack.clone()} 
                    chip={battlechip}
                    on_mouse_enter={self.chip_mouseover.clone()}
                />
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
            <div class="row sticky-top justify-content-center" style="background-color: gray">
                <div class="col-1 Chip nopadding"/>
                <div class="col-3 Chip nopadding" style="white-space: nowrap">
                    {"NAME"}
                </div>
                <div class="col-3 Chip nopadding">
                    {"SKILL"}
                </div>
                <div class="col-2 Chip nopadding"/>
                <div class="col-1 Chip nopadding">
                    {"U"}
                </div>
            </div>
        }
    }
}

