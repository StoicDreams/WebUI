use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use yew_agent::{Agent, AgentLink, Context, HandlerId};

#[derive(Serialize, Deserialize, Debug)]
pub(crate) enum Request {
    AppDrawerAgentMsg(usize),
}

pub(crate) struct AppDrawerAgent {
    link: AgentLink<AppDrawerAgent>,
    subscribers: HashSet<HandlerId>,
}

impl Agent for AppDrawerAgent {
    type Reach = Context<Self>;
    type Message = ();
    type Input = Request;
    type Output = usize;

    fn create(link: AgentLink<Self>) -> Self {
        Self {
            link,
            subscribers: HashSet::new(),
        }
    }

    fn update(&mut self, _msg: Self::Message) {}

    fn handle_input(&mut self, msg: Self::Input, _id: HandlerId) {
        match msg {
            Request::AppDrawerAgentMsg(s) => {
                log::info!("Agent:Toggle Left Drawer");
                for sub in self.subscribers.iter() {
                    self.link.respond(*sub, s);
                }
            }
        }
    }

    fn connected(&mut self, id: HandlerId) {
        self.subscribers.insert(id);
    }

    fn disconnected(&mut self, id: HandlerId) {
        self.subscribers.remove(&id);
    }
}
