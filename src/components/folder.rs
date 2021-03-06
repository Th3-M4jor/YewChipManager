use unchecked_unwrap::UncheckedUnwrap;
use yew::{prelude::*, agent::{Dispatcher, Dispatched}};
use yewtil::function_component;
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
        },
        group_folder::{
            GroupFldrMsgBus,
            GroupFldrAgentReq,
        },
    },
    util::{alert, list_spectators}
};

use web_sys::MouseEvent;
use wasm_bindgen::JsCast;
use std::rc::Rc;
use std::sync::atomic::Ordering;
#[derive(Properties, Clone)]
pub(crate) struct FolderProps {
    pub active: bool,
    pub in_folder_group: bool,
}

pub(crate) enum FolderMsg {
    ChangeSort(ChipSortOptions),
    ChangeUsed(usize),
    ReturnToPack(usize),
    SetHighlightedChip(usize),
    ChangeChipLimit(usize),
    JackOut,
    JoinFolerGroup,
    LeaveFolderGroup,
    ClearFolder,
    DoNothing,
}

pub(crate) struct FolderComponent {
    props: FolderProps,
    _link: ComponentLink<Self>,
    sort_by: ChipSortOptions,
    return_to_pack: Callback<MouseEvent>,
    change_used_callback: Callback<MouseEvent>,
    chip_mouseover: Callback<MouseEvent>,
    sort_change_callback: Callback<ChangeData>,
    event_bus: Dispatcher<GlobalMsgBus>,
    set_desc_bus: Dispatcher<ChipDescMsgBus>,
    chip_limit_change: Callback<ChangeData>,
    join_folder_group_callback: Callback<MouseEvent>,
    leave_folder_group_callback: Callback<MouseEvent>,
    jack_out_callback: Callback<MouseEvent>,
    clear_folder_callback: Callback<MouseEvent>,
}

fn mouse_event_to_index(e: MouseEvent) -> Option<usize> {
    let target = e.current_target()?;
    let div = target.dyn_ref::<web_sys::HtmlElement>()?;
    let id = div.id();
    id.get(3..)?.parse::<usize>().ok()
}

/// a chip was double clicked, return it to pack
fn return_pack_callback(e: MouseEvent) -> FolderMsg {
    let index = mouse_event_to_index(e);

    match index {
        Some(idx) => FolderMsg::ReturnToPack(idx),
        None => FolderMsg::DoNothing,
    }
}

/// function for fipping a chip between used/unused
fn change_used_callback_fn(e: MouseEvent) -> FolderMsg {
    
    let index = mouse_event_to_index(e);

    match index {
        Some(idx) => FolderMsg::ChangeUsed(idx),
        None => FolderMsg::DoNothing,
    }
}

/// function for setting the chip in the description box
fn handle_mouseover_event(e: MouseEvent) -> FolderMsg {
    
    let index = mouse_event_to_index(e);

    match index {
        Some(idx) => FolderMsg::SetHighlightedChip(idx),
        None => FolderMsg::DoNothing,
    }
    
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
        let chip_limit_change = link.callback(|e: ChangeData| {
            if let ChangeData::Value(text) = e {
                text.parse::<usize>().map_or(FolderMsg::DoNothing, |val| FolderMsg::ChangeChipLimit(val))
            } else {
                FolderMsg::DoNothing
            }
        });
        let chip_mouseover = link.callback(handle_mouseover_event);
        let set_desc_bus = ChipDescMsgBus::dispatcher();
        let event_bus = GlobalMsgBus::dispatcher();
        let join_folder_group_callback = link.callback(|_: MouseEvent| FolderMsg::JoinFolerGroup);
        let leave_folder_group_callback = link.callback(|_:MouseEvent| FolderMsg::LeaveFolderGroup);
        let jack_out_callback = link.callback(|_: MouseEvent| FolderMsg::JackOut);
        let clear_folder_callback = link.callback(|_: MouseEvent| FolderMsg::ClearFolder);

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
            chip_limit_change,
            leave_folder_group_callback,
            join_folder_group_callback,
            jack_out_callback,
            clear_folder_callback,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let updated = match msg {
            FolderMsg::ChangeSort(sort_by) => {
                if self.sort_by != sort_by {
                    self.sort_by = sort_by;
                    true
                } else {
                    false
                }
            },
            FolderMsg::ClearFolder => {
                let count = ChipLibrary::get_instance().clear_folder();
                let msg = count.to_string() + " chips have been returned to your pack";
                self.event_bus.send(GlobalMsgReq::SetHeaderMsg(msg));
                self.set_desc_bus.send(ChipDescMsg::ClearDesc);
                true
            },
            FolderMsg::JackOut => {
                let count = ChipLibrary::get_instance().jack_out();
                let msg = count.to_string() + " chips have been marked as unused";
                self.event_bus.send(GlobalMsgReq::SetHeaderMsg(msg));
                true
            },
            FolderMsg::ChangeChipLimit(val) => {
                match ChipLibrary::get_instance().update_chip_limit(val) {
                    Ok(should_update) => should_update,
                    Err(msg) => {
                        unsafe{alert(msg)};
                        true
                    }
                }
            },
            FolderMsg::ReturnToPack(idx) => self.return_chip_to_pack(idx),
            FolderMsg::ChangeUsed(idx) => {
                ChipLibrary::get_instance().flip_used_folder(idx);
                true
            },
            FolderMsg::SetHighlightedChip(idx) => self.set_highlighted_chip(idx),
            FolderMsg::DoNothing => false,
            FolderMsg::JoinFolerGroup => {
                self.event_bus.send(GlobalMsgReq::JoinGroup);
                false
            }
            FolderMsg::LeaveFolderGroup => {
                //self.event_bus.send(GlobalMsgReq::LeaveGroup);
                GroupFldrMsgBus::dispatcher().send(GroupFldrAgentReq::LeaveGroup);
                false
            }
        };
        if updated {
            GroupFldrMsgBus::dispatcher().send(GroupFldrAgentReq::UpdateFolder);
        }
        updated
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
        } else if props.in_folder_group != self.props.in_folder_group {
            self.props = props;
            return true;
        } else {
            return false;
        }
    }

    fn view(&self) -> Html {
        //let (folder_containter_class, outer_container_class) = if self.props.active {("container-fluid Folder activeFolder", "container-fluid")} else {("container-fluid Folder", "inactiveTab")};

        let (col1_display, col2_display, folder_containter_class) = if self.props.active {
            ("left-panel nopadding", "middle-panel nopadding", "container-fluid Folder activeFolder")
        } else {
            ("inactiveTab", "inactiveTab", "container-fluid Folder")
        };
        let lib_instance = ChipLibrary::get_instance();
        let chip_limit_val = lib_instance.chip_limit.load(Ordering::Relaxed).to_string();
        let min_val = unsafe{lib_instance.folder.try_borrow().unchecked_unwrap()}.len().to_string();
        
        html!{
            <>
            <div class=col1_display>
                <span unselectable="on" class="Chip noselect">{"Chip Limit:"}</span>
                <input 
                    type="number" class="chip-search-input"
                    min=min_val max="45"
                    value=chip_limit_val 
                    onchange=self.chip_limit_change.clone()
                    style="height: calc(1.5em + .75rem + 2px); font-size: 1rem"
                />
                <ChipSortBox sort_by=self.sort_by include_owned=false sort_changed=self.sort_change_callback.clone()/>
                <br/>
                <br/>
                {self.generate_buttons()}
                <br/>
                {list_spectators()}
            </div>
            <div class=col2_display>
                <div class=folder_containter_class>
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
        let mut folder = unsafe{ChipLibrary::get_instance().folder.try_borrow_mut().unchecked_unwrap()};
        if folder.len() == 0 {
            return html!{
                <span class="noselect Chip">
                {"Your folder is empty!"}
                </span>
            }
        }
        match self.sort_by {
            ChipSortOptions::Name => {
                folder.sort_by(|a, b| {
                    a.chip.class.cmp(&b.chip.class).then_with(||a.chip.name.cmp(&b.chip.name))
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

    fn generate_buttons(&self) -> Html {
        let (join_or_leave_text, join_or_leave_callback) = if self.props.in_folder_group {
            ("Leave folder group", self.leave_folder_group_callback.clone())
        } else {
            ("Join folder group", self.join_folder_group_callback.clone())
        };

        html!{
            <div class="centercontent">
                <button class="sideButtons ripple" onclick=self.jack_out_callback.clone()>
                    <span class="Chip">{"Jack Out"}</span>
                </button>
                <br/>
                <button class="sideButtons ripple" onclick=self.clear_folder_callback.clone()>
                    <span class="Chip">{"Clear Folder"}</span>
                </button>
                <br/>
                <button class="sideButtons ripple" onclick=join_or_leave_callback>
                    <span class="Chip">{join_or_leave_text}</span>
                </button>
                <br/>
            </div>
        }
    }

    fn set_highlighted_chip(&mut self, idx: usize) -> bool {
        let chip_library = ChipLibrary::get_instance();
        let folder = unsafe{chip_library.folder.try_borrow().unchecked_unwrap()};
        match folder.get(idx) {
            Some(chip) => {
                let name = chip.name.clone();
                self.set_desc_bus.send(ChipDescMsg::SetDesc(name));
            },
            None => {},
        }
        false
    }

    fn return_chip_to_pack(&mut self, idx: usize) -> bool {
        let chip_library = ChipLibrary::get_instance();
        let folder = unsafe{chip_library.folder.try_borrow().unchecked_unwrap()};
        let name = unsafe{folder.get(idx).unchecked_unwrap().name.as_str()};
        let msg = String::from("A copy of ") + name + " has been returned to your pack";
        self.event_bus.send(GlobalMsgReq::SetHeaderMsg(msg));
        drop(folder);
        if let Err(why) = chip_library.return_fldr_chip_to_pack(idx) {
            unsafe{alert(why)};
        }
        self.set_desc_bus.send(ChipDescMsg::ClearDesc);
        GroupFldrMsgBus::dispatcher().send(GroupFldrAgentReq::UpdateFolder);
        true
    }

}

#[function_component(FolderTopRow)]
pub(crate) fn folder_top_row() -> Html {
    html! {
        <div class="chip-top-row">
            <div class="chip-col-1 Chip nopadding">
                {"#"}
            </div>
            <div class="chip-col-3 Chip nopadding" style="white-space: nowrap">
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
            <div class="chip-col-1 Chip nopadding">
                {"U"}
            </div>
        </div>
    }
}