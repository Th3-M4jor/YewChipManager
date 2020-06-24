use serde::{Deserialize, Serialize};
use std::collections::{HashSet, HashMap};
use std::time::Duration;
use yew::worker::*;
//use yew::prelude::*;
use yew::format::Text;
use yew::services::{
    websocket::{WebSocketService, WebSocketTask, WebSocketStatus},
    interval::{IntervalService, IntervalTask},
};
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

impl From<std::option::NoneError> for GroupFldrAgentSocketMsg {
    fn from(_: std::option::NoneError) -> Self {
        GroupFldrAgentSocketMsg::DoNothing
    }
}

impl std::ops::Try for GroupFldrAgentSocketMsg {
    type Ok = Self;
    type Error = Self;

    fn into_result(self) -> Result<Self::Ok, Self::Error> {
        
        match self {
            GroupFldrAgentSocketMsg::DoNothing => Err(GroupFldrAgentSocketMsg::DoNothing),
            _ => Ok(self)
        }
    }
    fn from_error(_: Self::Error) -> Self {
        GroupFldrAgentSocketMsg::DoNothing
    }
    fn from_ok(v: Self::Ok) -> Self {
        v
    }
    
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub(crate) enum GroupFldrAgentOutMsg {
    JoinedGroup,
    LeftGroup,
    GroupUpdated,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) enum GroupFldrAgentReq {
    JoinGroup{player_name: String, group_name: String},
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
    socket_update_interval: Option<IntervalTask>,

}

//static GroupMsgCallbackLink: Lazy<RwLock<Option<Callback<GroupFldrAgentMsg>>>> = Lazy::new(|| RwLock::new(None));

impl Agent for GroupFldrMsgBus {
    type Reach = Context<Self>;
    type Message = GroupFldrAgentSocketMsg;
    type Input = GroupFldrAgentReq;
    type Output = GroupFldrAgentOutMsg;

    fn create(link: AgentLink<Self>) -> Self {
        Self {
            link,
            subs: HashSet::new(),
            web_socket: None,
            socket_update_interval: None,
        }
    }

    fn update(&mut self, msg: Self::Message) {
        let response = match msg {
            GroupFldrAgentSocketMsg::JoinedGroup => {
                let folder = ChipLibrary::get_instance().serialize_folder();
                match &mut self.web_socket {
                    Some(socket) => {
                        socket.send(Ok(folder));
                        let handle = IntervalService::spawn(
                            Duration::from_secs(10),
                            self.link.callback(|_| GroupFldrAgentSocketMsg::CheckFolderUpdated)
                        );
                        self.socket_update_interval = Some(handle);
                    },
                    None => {}
                }

                GroupFldrAgentOutMsg::JoinedGroup
            }
            GroupFldrAgentSocketMsg::LeftGroup => {
                self.web_socket.take();
                self.socket_update_interval.take();
                self.clear_group_folders();
                GroupFldrAgentOutMsg::LeftGroup
            }
            GroupFldrAgentSocketMsg::GroupUpdated => {
                GroupFldrAgentOutMsg::GroupUpdated
            }
            GroupFldrAgentSocketMsg::ServerError(why) => {
                unsafe{alert(&why)};
                self.web_socket.take();
                self.socket_update_interval.take();
                self.clear_group_folders();
                GroupFldrAgentOutMsg::LeftGroup
            }
            GroupFldrAgentSocketMsg::CheckFolderUpdated => {
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
            GroupFldrAgentReq::JoinGroup { player_name, group_name } => {
                if let Err(why) = self.join_group(group_name, player_name) {
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

    fn join_group(&mut self, group_name: String, player_name: String) -> Result<(), String> {
        let url = String::from("wss://spartan364.hopto.org/join/") + &group_name + "/" + &player_name;
        //let mut socket = WebSocketService::new();
        let message_callback = self.link.callback(|msg: Text| {
            let data = msg.ok()?;
            web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(&data));
            let res  = serde_json::from_str::<SocketMsg>(&data).ok()?;
            match res {
                SocketMsg::FoldersUpdated(folders) => {
                    //let folders = serde_json::from_str::<HashMap<String, Vec<GroupFolderChip>>>(&folder_str).ok()?;
                    let mut group = ChipLibrary::get_instance().group_folders.borrow_mut();
                    *group = folders;
                    GroupFldrAgentSocketMsg::GroupUpdated
                }
                SocketMsg::Error(why) => {
                    GroupFldrAgentSocketMsg::ServerError(why)
                }
                SocketMsg::Ready => {
                    GroupFldrAgentSocketMsg::JoinedGroup
                }
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
        let socket_task = WebSocketService::connect_text(&url, message_callback, socket_notification_callback).map_err(|e| e.to_owned())?;
        self.web_socket = Some(socket_task);

        Ok(())
    }

    fn leave_group(&mut self) {
        self.web_socket.take();
        self.socket_update_interval.take();
    }

    fn check_folder_upated(&mut self) {
        let library = ChipLibrary::get_instance();
        if !library.folder_changed() {
            return;
        }
        let folder = library.serialize_folder();
        match &mut self.web_socket {
            Some(socket) => socket.send(Ok(folder)),
            None => {}
        }
    }

    fn clear_group_folders(&self) {
        let mut group = ChipLibrary::get_instance().group_folders.borrow_mut();
        *group = HashMap::default()
    }

}