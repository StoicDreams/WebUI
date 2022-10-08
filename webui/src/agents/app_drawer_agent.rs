use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use yew_agent::{Agent, AgentLink, Context, HandlerId};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum AppDrawerRequest {
    ToggleTopDrawer(usize),
    ToggleRightDrawer(usize),
    ToggleBottomDrawer(usize),
    ToggleLeftDrawer(usize),
}

pub enum AppDrawerReceiverMessage {
    AppDrawerMessage(AppDrawerRequest),
    None,
}

pub(crate) struct AppDrawerAgent {
    link: AgentLink<AppDrawerAgent>,
    subscribers: HashSet<HandlerId>,
}

impl Agent for AppDrawerAgent {
    type Reach = Context<Self>;
    type Message = ();
    type Input = AppDrawerRequest;
    type Output = AppDrawerRequest;

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
