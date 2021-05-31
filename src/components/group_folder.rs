use yew::prelude::*;
use yew::agent::{Dispatcher, Dispatched};
use yew::services::ConsoleService;
use web_sys::MouseEvent;
use wasm_bindgen::JsCast;
use std::rc::Rc;
use crate::agents::{
    group_folder::*,
    chip_desc::*,
};
use crate::components::{folder::FolderTopRow, chips::GroupFolderChip};
use crate::chip_library::{ChipLibrary, BattleChip};
use crate::util::list_spectators;


#[derive(Properties, PartialEq, Clone)]
pub(crate) struct GroupFolderProps {
    pub player_name: String,
    pub active: bool,
}

pub(crate) enum GroupFolderComponentMsg {
    GroupFoldersUpdated,
    LeftGroup,
    DoNothing,
    SetHighlightedChip(String),
}

pub(crate) struct GroupFolderComponent {
    props: GroupFolderProps,
    _link: ComponentLink<Self>,
    _group_bridge: Box<dyn Bridge<GroupFldrMsgBus>>,
    set_desc_bus: Dispatcher<ChipDescMsgBus>,
    chip_mouseover: Callback<MouseEvent>,
}

impl Component for GroupFolderComponent {
    type Message = GroupFolderComponentMsg;
    type Properties = GroupFolderProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.callback(|e: GroupFldrAgentOutMsg |{
            match e {
                GroupFldrAgentOutMsg::JoinedGroup => GroupFolderComponentMsg::DoNothing,
                GroupFldrAgentOutMsg::LeftGroup => GroupFolderComponentMsg::LeftGroup,
                GroupFldrAgentOutMsg::GroupUpdated => GroupFolderComponentMsg::GroupFoldersUpdated,
            }
        });
        let _group_bridge = GroupFldrMsgBus::bridge(callback);
        let set_desc_bus = ChipDescMsgBus::dispatcher();
        let chip_mouseover = link.callback(handle_mouseover_event);
        Self {
            props,
            _link: link,
            _group_bridge,
            set_desc_bus,
            chip_mouseover,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            GroupFolderComponentMsg::GroupFoldersUpdated => true,
            GroupFolderComponentMsg::LeftGroup => true,
            GroupFolderComponentMsg::DoNothing => false,
            GroupFolderComponentMsg::SetHighlightedChip(name) => {
                self.set_desc_bus.send(ChipDescMsg::SetDesc(name));
                false
            }
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
        let (col1_display, col2_display, folder_containter_class) = if self.props.active {
            ("left-panel nopadding", "middle-panel nopadding", "container-fluid Folder activeFolder")
        } else {
            ("inactiveTab", "inactiveTab", "container-fluid Folder")
        };
        
        html!{
            <>
            <div class=col1_display>
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

impl GroupFolderComponent {
    fn build_folder(&self) -> Html {
        let library = ChipLibrary::get_instance();
        let group = match library.group_folders.try_borrow() {
            Ok(group) => group,
            Err(_) => {
                //web_sys::console::log_1(&wasm_bindgen::JsValue::from_str("Failed to immutably borrow group folders"));
                ConsoleService::log("Failed to immutably borrow group folders");
                return html!{};
            },
        };
        let folder = match group.get(&self.props.player_name) {
            Some(folder) => folder,
            None => return html!{},
        };

        //currently empty group folders are not shown
        
        /*
        if folder.len() == 0 {
            return html!{
                <span class="noselect Chip">
                    {"Their folder is empty!"}
                </span>
            }
        }
        */

        let folder_len = folder.len();
        folder.iter().zip(0..folder_len).map(|(chip, index)|{
            let battlechip = match library.library.get(&chip.name) {
                Some(chip) => chip.clone(),
                None => Rc::new(BattleChip::unknown_chip(&chip.name)),
            };
            html!{
                <GroupFolderChip
                    used={chip.used}
                    idx={index}
                    chip={battlechip}
                    on_mouse_enter={self.chip_mouseover.clone()}
                />
            }
        }).collect::<Html>()

    }

}

fn handle_mouseover_event(e: MouseEvent) -> GroupFolderComponentMsg {
    
    let id: Option<String> = e.current_target().as_ref().and_then(
        |target| target.dyn_ref::<web_sys::HtmlElement>()
    ).and_then(
        |div| Some(div.id())
    );
    
    match id {
        Some(id) => GroupFolderComponentMsg::SetHighlightedChip(id),
        None => GroupFolderComponentMsg::DoNothing,
    }
}