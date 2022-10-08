use crate::{
    agents::app_drawer_agent::{AppDrawerAgent, AppDrawerReceiverMessage, AppDrawerRequest},
    AppConfig, Direction, DrawerToggleInfo, Paper,
};
use yew::{html, Callback, Component, Context, Html, MouseEvent};
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
        let header_left_drawer_info = self.app_config.header_left_drawer_toggle.clone();
        let header_top_drawer_info = self.app_config.header_top_drawer_toggle.clone();
        let header_right_drawer_info = self.app_config.header_right_drawer_toggle.clone();
        let footer_left_drawer_info = self.app_config.footer_left_drawer_toggle.clone();
        let footer_bottom_drawer_info = self.app_config.footer_bottom_drawer_toggle.clone();
        let footer_right_drawer_info = self.app_config.footer_right_drawer_toggle.clone();

        let setup_onclick = |button_info: Option<DrawerToggleInfo>, drawer: Direction| {
            let onclick = ctx
                .link()
                .callback(move |_: MouseEvent| match button_info.clone() {
                    Some(drawer_info) => {
                        let message = drawer_info.drawer_content as usize;
                        match drawer {
                            Direction::Top => AppDrawerReceiverMessage::AppDrawerMessage(
                                AppDrawerRequest::ToggleTopDrawer(message.to_owned()),
                            ),
                            Direction::Right => AppDrawerReceiverMessage::AppDrawerMessage(
                                AppDrawerRequest::ToggleRightDrawer(message.to_owned()),
                            ),
                            Direction::Bottom => AppDrawerReceiverMessage::AppDrawerMessage(
                                AppDrawerRequest::ToggleBottomDrawer(message.to_owned()),
                            ),
                            Direction::Left => AppDrawerReceiverMessage::AppDrawerMessage(
                                AppDrawerRequest::ToggleLeftDrawer(message.to_owned()),
                            ),
                        }
                    }
                    None => AppDrawerReceiverMessage::None,
                });
            onclick
        };

        html! {
            <header>
                {match header_left_drawer_info.clone() {
                    Some(drawer_info) => {
                        html! {
                            <button type="button" title={drawer_info.title} class="logo"
                                onclick={ctx.link().callback(move |_| {
                                    let message = drawer_info.drawer_content as usize;
                                    AppDrawerReceiverMessage::AppDrawerMessage(
                                        AppDrawerRequest::ToggleLeftDrawer(
                                            message.to_owned()
                                        )
                                    )
                                })}>
                                {match &self.app_config.header_logo_src {
                                    Some(logo) => {
                                        html! {
                                            <img src={logo.to_string()} title={format!("{} logo", self.app_config.company_name)} />
                                        }
                                    },
                                    None => html! {}
                                }}
                                <span class="btn toggle elevation-1">{(drawer_info.display)()}</span>
                            </button>
                        }
                    },
                    None => html! {
                        {match &self.app_config.header_logo_src {
                            Some(logo) => {
                                html! {
                                    <img src={logo.to_string()} title={format!("{} logo", self.app_config.company_name)} />
                                }
                            },
                            None => html! {}
                        }}
                    }
                }}
                <h1 class="flex-grow">{ self.app_config.app_name.clone() }</h1>
                {render_drawer_button(
                    header_top_drawer_info.clone(),
                    setup_onclick(header_top_drawer_info.clone(), Direction::Top)
                )}
                <Paper class="d-flex justify-right">
                    {"Guest"}
                </Paper>
            </header>
        }
    }
}

fn render_drawer_button(
    button_info: Option<DrawerToggleInfo>,
    onclick: yew::Callback<MouseEvent>,
) -> Html {
    match button_info {
        Some(drawer_info) => {
            html! {
                <button type="button" title={drawer_info.title} class="btn toggle elevation-1" onclick={onclick}>
                    {(drawer_info.display)()}
                </button>
            }
        }
        None => html! {},
    }
}
