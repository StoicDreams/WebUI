use crate::yew_agent::{Agent, AgentLink, Context, HandlerId};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum AppStateRequest {
    PathUpdate(String),
}

pub enum AppStateReceiverMessage {
    AppStateMessage(AppStateRequest),
    None,
}

#[derive(Clone)]
pub struct AppStateAgent {
    link: AgentLink<AppStateAgent>,
    subscribers: HashSet<HandlerId>,
}

impl Agent for AppStateAgent {
    type Reach = Context<Self>;
    type Message = ();
    type Input = AppStateRequest;
    type Output = AppStateRequest;

    fn create(link: AgentLink<Self>) -> Self {
        Self {
            link,
            subscribers: HashSet::new(),
        }
    }

    fn update(&mut self, _msg: Self::Message) {}

    fn handle_input(&mut self, msg: Self::Input, _id: HandlerId) {
        for sub in self.subscribers.iter() {
            self.link.respond(*sub, msg.clone());
        }
    }

    fn connected(&mut self, id: HandlerId) {
        self.subscribers.insert(id);
    }

    fn disconnected(&mut self, id: HandlerId) {
        self.subscribers.remove(&id);
    }
}
