use crate::yew_agent::{Agent, AgentLink, Context, HandlerId};
use crate::*;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct AppDrawerOptions {
    pub(crate) title: String,
    pub(crate) display_ref: usize,
    pub(crate) hide_header: bool,
    pub(crate) hide_footer: bool,
    pub(crate) hide_close_x: bool,
}
pub struct AppDrawerOptionsBuilder {
    title: String,
    display_ref: usize,
    hide_header: bool,
    hide_footer: bool,
    hide_close_x: bool,
}

impl AppDrawerOptionsBuilder {
    pub fn build(self: Self) -> AppDrawerOptions {
        AppDrawerOptions {
            title: self.title,
            display_ref: self.display_ref,
            hide_header: self.hide_header,
            hide_footer: self.hide_footer,
            hide_close_x: self.hide_close_x,
        }
    }
    pub fn hide_close_x(self: &mut Self) -> &mut AppDrawerOptionsBuilder {
        self.hide_close_x = true;
        self
    }
    pub fn hide_header(self: &mut Self) -> &mut AppDrawerOptionsBuilder {
        self.hide_header = true;
        self
    }
    pub fn hide_footer(self: &mut Self) -> &mut AppDrawerOptionsBuilder {
        self.hide_footer = true;
        self
    }
}

impl AppDrawerOptions {
    pub fn new(title: String, display: fn() -> Html) -> AppDrawerOptionsBuilder {
        AppDrawerOptionsBuilder {
            title,
            display_ref: display as usize,
            hide_header: false,
            hide_footer: false,
            hide_close_x: false,
        }
    }
    pub(crate) fn get_display(self: Self) -> fn() -> Html {
        let content: fn() -> Html = if self.display_ref > 0 {
            let fnptr = self.display_ref as *const ();
            unsafe { std::mem::transmute(fnptr) }
        } else {
            || html!("")
        };
        content
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum AppDrawerRequest {
    ToggleTopDrawer(Option<AppDrawerOptions>),
    ToggleRightDrawer(Option<AppDrawerOptions>),
    ToggleBottomDrawer(Option<AppDrawerOptions>),
    ToggleLeftDrawer(Option<AppDrawerOptions>),
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
