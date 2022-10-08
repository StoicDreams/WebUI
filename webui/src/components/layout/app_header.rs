use crate::{
    agents::app_drawer_agent::{AppDrawerAgent, AppDrawerReceiverMessage, AppDrawerRequest},
    AppConfig, Paper,
};
use yew::{html, Callback, Component, Context, Html};
use yew_agent::{Dispatched, Dispatcher};

pub(crate) struct AppHeader {
    app_drawer_agent: Dispatcher<AppDrawerAgent>,
    app_config: AppConfig,
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
            AppDrawerReceiverMessage::None => false,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let left_drawer_info = self.app_config.header_left_drawer_toggle.clone();
        html! {
            <header>
                <button class="logo" onclick={
                    ctx.link().callback(
                        move |_| {
                            match left_drawer_info.clone() {
                                Some(drawer_info) => {
                                    let message = drawer_info.drawer_content as usize;
                                    AppDrawerReceiverMessage::AppDrawerMessage(
                                        AppDrawerRequest::ToggleLeftDrawer(
                                            message.to_owned()
                                        )
                                    )
                                },
                                None => AppDrawerReceiverMessage::None,
                            }
                        }
                    )}>
                    {match &self.app_config.header_logo_src {
                        Some(logo) => {
                            html! {
                                <img src={logo.to_string()} title={format!("{} logo", self.app_config.company_name)} />
                            }
                        },
                        None => html! {}
                    }}
                    {match self.app_config.header_left_drawer_toggle.clone() {
                        Some(drawer_info) => {
                            html! {
                                <button class="btn toggle elevation-1">
                                    {(drawer_info.display)()}
                                </button>
                            }
                        },
                        None => html! {}
                    }}
                </button>
                <Paper class="grow">{ self.app_config.app_name.clone() }</Paper>
                {match self.app_config.header_top_drawer_toggle.clone() {
                    Some(drawer_info) => {
                        html! {
                            <button class="btn toggle elevation-1">
                                {(drawer_info.display)()}
                            </button>
                        }
                    },
                    None => html! {}
                }}
            </header>
        }
    }
}
