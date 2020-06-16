use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use yew::worker::*;
use web_sys::WebSocket;

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub(crate) enum GroupFldrAgentMsg {
    JoinedGroup,
    LeftGroup,
    GroupUpdated,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) enum GroupFldrAgentReq {
    JoinGroup{player_name: String, group_name: String},
    LeaveGroup,
    UpdateFolder,
}

pub(crate) struct GroupFldrMsgBus {
    link: AgentLink<Self>,
    subs: HashSet<HandlerId>,
    web_socket: Option<WebSocket>,
}

impl Agent for GroupFldrMsgBus {
    type Reach = Context;
    type Message = GroupFldrAgentMsg;
    type Input = GroupFldrAgentReq;
    type Output = GroupFldrAgentMsg;

    fn create(link: AgentLink<Self>) -> Self {
        Self {
            link,
            subs: HashSet::new(),
            web_socket: None,
        }
    }

    fn update(&mut self, msg: Self::Message) {
        match msg {
            GroupFldrAgentMsg::JoinedGroup => {}
            GroupFldrAgentMsg::LeftGroup => {
                self.web_socket = None;
            }
            GroupFldrAgentMsg::GroupUpdated => {}
        }
        for sub in self.subs.iter() {
            self.link.respond(*sub, msg);
        }
    }

    fn handle_input(&mut self, msg: Self::Input, _id: HandlerId) {
        match msg {
            GroupFldrAgentReq::JoinGroup { player_name, group_name } => {
                self.join_group(group_name, player_name);
            }
            GroupFldrAgentReq::LeaveGroup => {
                self.leave_group();
            }
            GroupFldrAgentReq::UpdateFolder => {
                self.folder_upated();
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

    fn join_group(&mut self, _group_name: String, _player_name: String) {
        todo!()
    }

    fn leave_group(&self) {
        todo!()
    }

    fn folder_upated(&self) {
        todo!()
    }

}