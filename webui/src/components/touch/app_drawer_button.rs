use crate::AppDrawerOptions;
use crate::yew::Context;
use crate::{
    agents::app_drawer_agent::{AppDrawerAgent, AppDrawerReceiverMessage, AppDrawerRequest},
    html, Children, Component, Direction, Dispatched, Dispatcher, DrawerToggleInfo, Html,
    MouseEvent, Paper, Properties,
};

/// Properties for NavLink component
#[derive(Properties, PartialEq)]
pub struct AppDrawerButtonProps {
    pub info: Option<DrawerToggleInfo>,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: String,
    #[prop_or_default]
    pub logosrc: Option<String>,
    #[prop_or_default]
    pub logotitle: Option<String>,
    #[prop_or_default]
    pub always_show_logo: bool,
}

/// Button that is used to trigger opening one of the four app drawers.
///
/// Left and right app drawers are side panels that pop out with a width dependent on their content.
/// Top and Bottom app drawers act more like dialogs|modals, sliding out and displaying in the center of the page.
pub struct AppDrawerButton {
    app_drawer_agent: Dispatcher<AppDrawerAgent>,
    drawer_info: Option<DrawerToggleInfo>,
    logo_src: Option<String>,
    logo_title: String,
}

impl Component for AppDrawerButton {
    type Message = AppDrawerReceiverMessage;
    type Properties = AppDrawerButtonProps;

    fn create(ctx: &Context<Self>) -> Self {
        let props = ctx.props();
        Self {
            app_drawer_agent: AppDrawerAgent::dispatcher(),
            drawer_info: props.info.to_owned(),
            logo_src: props.logosrc.clone(),
            logo_title: props.logotitle.clone().unwrap_or("".to_owned()),
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
        let props = ctx.props();
        let drawer_info = self.drawer_info.clone();
        let setup_onclick = {
            let onclick = ctx
                .link()
                .callback(move |_: MouseEvent| match drawer_info.clone() {
                    Some(drawer_info) => {
                        let message = AppDrawerOptions::new(
                            drawer_info.title.to_owned(),
                            drawer_info.drawer_content,
                        ).build();
                        match drawer_info.drawer {
                            Direction::Top => AppDrawerReceiverMessage::AppDrawerMessage(
                                AppDrawerRequest::ToggleTopDrawer(Some(message)),
                            ),
                            Direction::Right => AppDrawerReceiverMessage::AppDrawerMessage(
                                AppDrawerRequest::ToggleRightDrawer(Some(message)),
                            ),
                            Direction::Bottom => AppDrawerReceiverMessage::AppDrawerMessage(
                                AppDrawerRequest::ToggleBottomDrawer(Some(message)),
                            ),
                            Direction::Left => AppDrawerReceiverMessage::AppDrawerMessage(
                                AppDrawerRequest::ToggleLeftDrawer(Some(message)),
                            ),
                        }
                    }
                    None => AppDrawerReceiverMessage::None,
                });
            onclick
        };

        html! {
            <>
                {match self.drawer_info.clone() {
                    Some(drawer_info) => {
                        html! {
                            <button type="button" title={drawer_info.title} class={props.class.to_owned()}
                                onclick={setup_onclick}>
                                {match &self.logo_src {
                                    Some(logo) => {
                                        html! {
                                            <img src={logo.to_string()} title={self.logo_title.to_owned()} />
                                        }
                                    },
                                    None => html! {}
                                }}
                                <span class="btn toggle elevation-1">{(drawer_info.display)()}</span>
                                {for ctx.props().children.iter()}
                            </button>
                        }
                    },
                    None => html! {
                        if props.always_show_logo {
                            {match &self.logo_src {
                                Some(logo) => {
                                    html! {
                                        <Paper>
                                            <img src={logo.to_string()} title={self.logo_title.to_owned()} />
                                        </Paper>
                                    }
                                },
                                None => html! {}
                            }}
                        }
                    }
                }}
            </>
        }
    }
}
