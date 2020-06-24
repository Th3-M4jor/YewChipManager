use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use yew::worker::*;

#[derive(Serialize, Deserialize, Clone)]
pub(crate) enum ChipDescMsg {
    SetDesc(String),
    ClearDesc,
}

pub(crate) struct ChipDescMsgBus {
    link: AgentLink<Self>,
    subs: HashSet<HandlerId>,
}

impl Agent for ChipDescMsgBus {
    type Reach = Context<Self>;
    type Message = ();
    type Input = ChipDescMsg;
    type Output = ChipDescMsg;

    fn create(link: AgentLink<Self>) -> Self {
        Self {
            link,
            subs: HashSet::new(),
        }
    }

    fn update(&mut self, _: Self::Message) {}

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