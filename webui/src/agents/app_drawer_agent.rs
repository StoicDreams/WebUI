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
    pub(crate) hide_cancel: bool,
    pub(crate) on_confirm: Option<usize>,
    pub(crate) confirm_display: String,
}
pub struct AppDrawerOptionsBuilder {
    title: String,
    display_ref: usize,
    hide_header: bool,
    hide_footer: bool,
    hide_close_x: bool,
    hide_cancel: bool,
    on_confirm: Option<fn() -> bool>,
    confirm_display: String,
}

impl AppDrawerOptionsBuilder {
    pub fn build(self: Self) -> AppDrawerOptions {
        AppDrawerOptions {
            title: self.title,
            display_ref: self.display_ref,
            hide_header: self.hide_header,
            hide_footer: self.hide_footer,
            hide_close_x: self.hide_close_x,
            hide_cancel: self.hide_cancel,
            on_confirm: match self.on_confirm {
                Some(method) => Some(method as usize),
                None => None,
            },
            confirm_display: self.confirm_display.to_string(),
        }
    }
    pub fn hide_close_x(self: &mut Self) -> &mut Self {
        self.hide_close_x = true;
        self
    }
    pub fn hide_header(self: &mut Self) -> &mut Self {
        self.hide_header = true;
        self
    }
    pub fn hide_footer(self: &mut Self) -> &mut Self {
        self.hide_footer = true;
        self
    }
    pub(crate) fn hide_cancel(self: &mut Self) -> &mut Self {
        self.hide_cancel = true;
        self
    }
    pub fn set_on_confirm(self: &mut Self, display: String, on_confirm: fn() -> bool) -> &mut Self {
        self.on_confirm = Some(on_confirm);
        self.confirm_display = display;
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
            hide_cancel: false,
            confirm_display: "Confirm".to_string(),
            on_confirm: None,
        }
    }

    pub(crate) fn get_display(self: &Self) -> fn() -> Html {
        let content: fn() -> Html = if self.display_ref > 0 {
            let fnptr = self.display_ref as *const ();
            unsafe { std::mem::transmute(fnptr) }
        } else {
            || html!("")
        };
        content
    }

    pub(crate) fn get_on_confirm(self: &Self) -> fn() -> bool {
        match self.on_confirm {
            Some(value) => {
                let content: fn() -> bool = if value > 0 {
                    let fnptr = value as *const ();
                    unsafe { std::mem::transmute(fnptr) }
                } else {
                    || true
                };
                content
            }
            None => || true,
        }
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
