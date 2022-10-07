use crate::{
    agents::app_drawer_agent::{AppDrawerAgent, AppDrawerReceiverMessage, AppDrawerRequest},
    AppConfig, NavLink, Paper,
};
use yew::{html, Callback, Component, Context, Html};
use yew_agent::{Dispatched, Dispatcher};
use yew_router::Routable;

pub(crate) struct AppHeader {
    app_drawer_agent: Dispatcher<AppDrawerAgent>,
    app_config: AppConfig,
}

/// Website route handling definition
#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/about")]
    About,
}

impl Component for AppHeader {
    type Message = AppDrawerReceiverMessage;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let (app_config, _) = _ctx
            .link()
            .context::<AppConfig>(Callback::noop())
            .expect("no app config found");
        Self {
            app_config,
            app_drawer_agent: AppDrawerAgent::dispatcher(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            AppDrawerReceiverMessage::AppDrawerMessage(message) => {
                self.app_drawer_agent.send(message);
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        //js! {
        //    console.log("Hello World");
        //};
        html! {
            <header>
                {match self.app_config.header_left_drawer_toggle.clone() {
                    Some(drawer_info) => {
                        let message = drawer_info.drawer_content as usize;
                        html! {
                            <a class="btn toggle elevation-1" onclick={ctx.link().callback(move |_| AppDrawerReceiverMessage::AppDrawerMessage(AppDrawerRequest::ToggleLeftDrawer(message.to_owned())))}>
                                {(drawer_info.display)()}
                            </a>
                        }
                    },
                    None => html! {""}
                }}
                <Paper>{ self.app_config.app_name.clone() }</Paper>
            </header>
        }
    }
}
