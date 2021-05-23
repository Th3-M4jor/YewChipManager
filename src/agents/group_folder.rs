use serde::{Deserialize, Serialize};
use std::collections::{HashSet, HashMap};
use std::time::Duration;
use yew::{Callback, worker::*};
//use yew::prelude::*;
use yew::format::Binary;
use yew_services::{
    websocket::{WebSocketService, WebSocketTask, WebSocketStatus},
    timeout::{TimeoutService, TimeoutTask},
    ConsoleService,
};
use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};
use unchecked_unwrap::UncheckedUnwrap;
const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b':').add(b'/').add(b'?').add(b'#').add(b'[')
                                        .add(b']').add(b'@').add(b'!').add(b'$').add(b'&').add(b'\'')
                                        .add(b'(').add(b')').add(b'*').add(b'+').add(b',').add(b';')
                                        .add(b'=').add(b'%');
//use wasm_bindgen::{JsCast, JsValue, closure::Closure};
//use web_sys::WebSocket;

use crate::util::alert;
use crate::chip_library::{GroupFolderChip, ChipLibrary};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) enum GroupFldrAgentSocketMsg {
    JoinedGroup,
    LeftGroup,
    GroupUpdated,
    ServerError(String),
    CheckFolderUpdated,
    DoNothing,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub(crate) enum GroupFldrAgentOutMsg {
    JoinedGroup,
    LeftGroup,
    GroupUpdated,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) enum GroupFldrAgentReq {
    JoinGroup{player_name: String, group_name: String, spectator: bool},
    UpdateFolder,
    LeaveGroup,
}

#[derive(Serialize, Deserialize, Clone)]
pub(crate) enum SocketMsg {
    FoldersUpdated(HashMap<String, Vec<GroupFolderChip>>),
    Error(String),
    Ready,
}

pub(crate) struct GroupFldrMsgBus {
    link: AgentLink<Self>,
    subs: HashSet<HandlerId>,
    web_socket: Option<WebSocketTask>,
    socket_update_timeout: Option<TimeoutTask>,
    timeout_callback: Callback<()>,
    spectator: bool
}

//static GroupMsgCallbackLink: Lazy<RwLock<Option<Callback<GroupFldrAgentMsg>>>> = Lazy::new(|| RwLock::new(None));

impl Agent for GroupFldrMsgBus {
    type Reach = Context<Self>;
    type Message = GroupFldrAgentSocketMsg;
    type Input = GroupFldrAgentReq;
    type Output = GroupFldrAgentOutMsg;

    fn create(link: AgentLink<Self>) -> Self {
        let callback = link.callback(|()| {
            GroupFldrAgentSocketMsg::CheckFolderUpdated   
        });
        Self {
            link,
            subs: HashSet::new(),
            web_socket: None,
            socket_update_timeout: None,
            timeout_callback: callback,
            spectator: false,
        }
    }

    fn update(&mut self, msg: Self::Message) {
        let response = match msg {
            GroupFldrAgentSocketMsg::JoinedGroup => {
                let folder = if self.spectator {
                    let fake_fldr: Vec<GroupFolderChip> = Vec::new();
                    unsafe{bincode::serialize(&fake_fldr).unchecked_unwrap()}
                } else {
                    ChipLibrary::get_instance().serialize_folder()
                };

                match &mut self.web_socket {
                    Some(socket) => {
                        socket.send_binary(Ok(folder));
                    },
                    None => {}
                }
                GroupFldrAgentOutMsg::JoinedGroup
            }
            GroupFldrAgentSocketMsg::LeftGroup => {
                self.web_socket.take();
                self.socket_update_timeout.take();
                self.clear_group_folders();
                self.spectator = false;
                GroupFldrAgentOutMsg::LeftGroup
            }
            GroupFldrAgentSocketMsg::GroupUpdated => {
                GroupFldrAgentOutMsg::GroupUpdated
            }
            GroupFldrAgentSocketMsg::ServerError(why) => {
                unsafe{alert(&why)};
                self.web_socket.take();
                self.spectator = false;
                self.socket_update_timeout.take();
                self.clear_group_folders();
                GroupFldrAgentOutMsg::LeftGroup
            }
            GroupFldrAgentSocketMsg::CheckFolderUpdated => {
                self.socket_update_timeout.take();
                self.check_folder_upated();
                return;
            },
            GroupFldrAgentSocketMsg::DoNothing => return,
            
        };
        for sub in self.subs.iter() {
            self.link.respond(*sub, response);
        }
    }

    fn handle_input(&mut self, msg: Self::Input, _id: HandlerId) {
        match msg {
            GroupFldrAgentReq::JoinGroup { player_name, group_name, spectator } => {
                if let Err(why) = self.join_group(group_name, player_name, spectator) {
                    unsafe{alert(&why)};
                }
            }
            GroupFldrAgentReq::LeaveGroup => {
                self.leave_group();
                self.clear_group_folders();
                for sub in self.subs.iter() {
                    self.link.respond(*sub, GroupFldrAgentOutMsg::LeftGroup);
                }
            }
            GroupFldrAgentReq::UpdateFolder => {
                if self.web_socket.is_none() {
                    return;
                }
                self.check_folder_upated();
            }
        }
    }

    fn connected(&mut self, id: HandlerId) {
        self.subs.insert(id);
    }

    fn disconnected(&mut self, id: HandlerId) {
        self.subs.remove(&id);
    }
}

impl GroupFldrMsgBus {

    fn join_group(&mut self, group_name: String, player_name: String, spectator: bool) -> Result<(), String> {
        let encoded_group = utf8_percent_encode(&group_name, FRAGMENT).to_string();
        let encoded_player = utf8_percent_encode(&player_name, FRAGMENT).to_string();
        let url = String::from("wss://spartan364.hopto.org/manager/api/join/") + &encoded_group + "/" + &encoded_player;
        //let mut socket = WebSocketService::new();
        let message_callback = self.link.callback(|msg: Binary| {
            let data = match msg {
                Ok(data) => data,
                Err(_) => return GroupFldrAgentSocketMsg::DoNothing,
            };
            //web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(&data));
            let res  = bincode::deserialize::<SocketMsg>(&data).ok();
            match res {
                Some(SocketMsg::FoldersUpdated(folders)) => {
                    //let folders = serde_json::from_str::<HashMap<String, Vec<GroupFolderChip>>>(&folder_str).ok()?;
                    //web_sys::console::log_1(&wasm_bindgen::JsValue::from_str("Folders updated"));
                    folders_updated(folders);
                    /*
                    let mut group = ChipLibrary::get_instance().group_folders.borrow_mut();
                    *group = folders;
                    drop(group);
                    */
                    GroupFldrAgentSocketMsg::GroupUpdated
                }
                Some(SocketMsg::Error(why)) => {
                    GroupFldrAgentSocketMsg::ServerError(why)
                }
                Some(SocketMsg::Ready) => {
                    GroupFldrAgentSocketMsg::JoinedGroup
                }
                None => GroupFldrAgentSocketMsg::DoNothing
            }
            //GroupFldrAgentMsg::GroupUpdated

        });
        let socket_notification_callback = self.link.callback(|msg: WebSocketStatus| {
            match msg {
                WebSocketStatus::Opened => {
                    //web_sys::console::log_1(&wasm_bindgen::JsValue::from_str("Socket opened"));
                    GroupFldrAgentSocketMsg::DoNothing
                },
                WebSocketStatus::Closed => GroupFldrAgentSocketMsg::LeftGroup,
                WebSocketStatus::Error => GroupFldrAgentSocketMsg::ServerError("Socket Closed by Server".to_string()),
            }
        });
        let socket_task = WebSocketService::connect_binary(&url, message_callback, socket_notification_callback).map_err(|e| e.to_string())?;
        self.web_socket = Some(socket_task);
        self.spectator = spectator;
        Ok(())
    }

    fn leave_group(&mut self) {
        self.web_socket.take();
        self.socket_update_timeout.take();
    }

    fn check_folder_upated(&mut self) {
        let library = ChipLibrary::get_instance();
        if !library.folder_changed() {
            return;
        }

        if self.socket_update_timeout.is_some() {
            return;
        }

        let folder = library.serialize_folder();
        match &mut self.web_socket {
            Some(socket) => socket.send_binary(Ok(folder)),
            None => {}
        }

        let timeout = TimeoutService::spawn(
            Duration::from_secs(1),
            self.timeout_callback.clone(),
        );
        self.socket_update_timeout = Some(timeout);

    }

    fn clear_group_folders(&self) {
        folders_updated(HashMap::default());
    }

}

fn folders_updated(new_folders: HashMap<String, Vec<GroupFolderChip>>) -> bool {
    let mut folders = match ChipLibrary::get_instance().group_folders.try_borrow_mut() {
        Ok(folders) => folders,
        Err(_) => {
            //web_sys::console::log_1(&wasm_bindgen::JsValue::from_str("Folder update failed"));
            ConsoleService::log("Folder update failed");
            return false;
        },
    };

    //web_sys::console::log_1(&wasm_bindgen::JsValue::from_str("Folders updated"));
    *folders = new_folders;
    return true;
}