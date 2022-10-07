use crate::agents::app_drawer_agent::{AppDrawerAgent, AppDrawerReceiverMessage, AppDrawerRequest};
use yew::{html, Component, Html, Properties};
use yew_agent::{Bridge, Bridged};

/// Properties for app drawer components
#[derive(Properties, PartialEq)]
pub(crate) struct AppDrawerProps {
    pub class: String,
}

#[derive(Default, Copy, Clone, PartialEq, Debug)]
pub(crate) struct AppDrawerState {
    pub is_open: bool,
    pub content: usize,
}

pub(crate) struct AppDrawer {
    app_drawer_agent: Box<dyn Bridge<AppDrawerAgent>>,
    is_open: bool,
    content: usize,
}

impl AppDrawer {
    fn toggle_state(&mut self, content_ref: usize) {
        self.is_open = content_ref > 0 && !self.is_open;
        if content_ref > 0 {
            self.content = content_ref.clone();
        }
    }
}

impl Component for AppDrawer {
    type Message = AppDrawerReceiverMessage;
    type Properties = AppDrawerProps;

    fn create(ctx: &yew::Context<Self>) -> Self {
        Self {
            app_drawer_agent: AppDrawerAgent::bridge(
                ctx.link()
                    .callback(AppDrawerReceiverMessage::AppDrawerMessage),
            ),
            is_open: false,
            content: 0,
        }
    }

    fn update(&mut self, ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        let is_open = self.is_open.clone();
        match msg {
            AppDrawerReceiverMessage::AppDrawerMessage(message) => {
                match message {
                    AppDrawerRequest::ToggleTopDrawer(fnval) => {
                        if ctx.props().class != "top" {
                            return false;
                        }
                        self.toggle_state(fnval);
                    }
                    AppDrawerRequest::ToggleRightDrawer(fnval) => {
                        if ctx.props().class != "right" {
                            return false;
                        }
                        self.toggle_state(fnval);
                    }
                    AppDrawerRequest::ToggleBottomDrawer(fnval) => {
                        if ctx.props().class != "bottom" {
                            return false;
                        }
                        self.toggle_state(fnval);
                    }
                    AppDrawerRequest::ToggleLeftDrawer(fnval) => {
                        if ctx.props().class != "left" {
                            return false;
                        }
                        self.toggle_state(fnval);
                    }
                }
                is_open != self.is_open
            }
            AppDrawerReceiverMessage::None => false,
        }
    }

    fn view(&self, ctx: &yew::Context<Self>) -> Html {
        let props = ctx.props();
        let class = format!(
            "app-drawer {} {}",
            props.class,
            if self.is_open { "open" } else { "closed" }
        );
        let content: fn() -> Html = if self.content > 0 {
            let fnptr = self.content as *const ();
            unsafe { std::mem::transmute(fnptr) }
        } else {
            || html!("")
        };
        html! {
            <aside class={class}>
                <div class="page-cover" onclick={ctx.link().callback(move |_| AppDrawerReceiverMessage::AppDrawerMessage(AppDrawerRequest::ToggleLeftDrawer(0.to_owned())))}>
                </div>
                <div class="drawer-placement">
                    <div class="drawer-content">
                        {content()}
                    </div>
                </div>
            </aside>
        }
    }
}
