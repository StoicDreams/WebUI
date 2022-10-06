use crate::{
    agents::app_drawer_agent::{AppDrawerAgent, Request},
    AppConfig, Paper,
};
use yew::{html, Callback, Component, Context, Html};
use yew_agent::{Dispatched, Dispatcher};

pub(crate) enum HeaderAction {
    ToggleTopDrawer(usize),
    ToggleRightDrawer(usize),
    ToggleBottomDrawer(usize),
    ToggleLeftDrawer(usize),
}

pub(crate) struct AppHeader {
    event_bus: Dispatcher<AppDrawerAgent>,
    app_config: AppConfig,
}

impl Component for AppHeader {
    type Message = HeaderAction;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let (app_config, _) = _ctx
            .link()
            .context::<AppConfig>(Callback::noop())
            .expect("no app config found");
        Self {
            app_config,
            event_bus: AppDrawerAgent::dispatcher(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            HeaderAction::ToggleLeftDrawer(message) => {
                log::info!("Header -> Toggle Left Drawer");
                self.event_bus
                    .send(Request::AppDrawerAgentMsg(message.to_owned()));
                false
            }
            HeaderAction::ToggleTopDrawer(_) => todo!(),
            HeaderAction::ToggleRightDrawer(_) => todo!(),
            HeaderAction::ToggleBottomDrawer(_) => todo!(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <header>
                {match self.app_config.header_left_drawer_toggle.clone() {
                    Some(drawer_info) => {
                        let message = drawer_info.drawer_content as usize;
                        html! {
                            <a class="btn toggle elevation-1" onclick={ctx.link().callback(move |_| HeaderAction::ToggleLeftDrawer(message.to_owned()))}>
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
