use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use yew::worker::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) enum Request {
    SetHeaderMsg(String),
    JoinGroup,
    EraseData,
    ImportData,
}

pub(crate) struct GlobalMsgBus {
    link: AgentLink<Self>,
    subs: HashSet<HandlerId>,
}

impl Agent for GlobalMsgBus {
    type Reach = Context<Self>;
    type Message = ();
    type Input = Request;
    type Output = Request;

    fn create(link: AgentLink<Self>) -> Self {
        Self {
            link,
            subs: HashSet::new(),
        }
    }

    fn update(&mut self, _msg: Self::Message) {}

    fn handle_input(&mut self, msg: Self::Input, _id: HandlerId) {
        for sub in self.subs.iter() {
            self.link.respond(*sub, msg.clone());
        }
    }

    fn connected(&mut self, id: HandlerId) {
        self.subs.insert(id);
    }

    fn disconnected(&mut self, id: HandlerId) {
        self.subs.remove(&id);
    }

}