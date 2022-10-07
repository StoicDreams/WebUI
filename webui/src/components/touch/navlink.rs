use crate::{
    app_drawer_agent::{AppDrawerAgent, AppDrawerRequest},
    app_state_agent::{AppStateAgent, AppStateReceiverMessage, AppStateRequest},
    interop,
};
use yew::{html, Children, Classes, Component, Context, Html, Properties};
use yew_agent::{Bridge, Bridged, Dispatched, Dispatcher};

/// Properties for NavLink component
#[derive(Properties, PartialEq)]
pub struct NavLinkProps {
    #[prop_or_default]
    pub to: String,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: String,
}

pub struct NavLink {
    app_state_agent: Box<dyn Bridge<AppStateAgent>>,
    app_drawer_agent: Dispatcher<AppDrawerAgent>,
    is_active: bool,
}

impl Component for NavLink {
    type Message = AppStateReceiverMessage;
    type Properties = NavLinkProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            app_state_agent: AppStateAgent::bridge(
                ctx.link()
                    .callback(AppStateReceiverMessage::AppStateMessage),
            ),
            app_drawer_agent: AppDrawerAgent::dispatcher(),
            is_active: interop::is_current_path(ctx.props().to.to_owned()),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            AppStateReceiverMessage::AppStateMessage(request) => {
                let to_send = request.clone();
                match request {
                    AppStateRequest::PathUpdate(path) => {
                        if ctx.props().to.to_lowercase() == path.to_lowercase() {
                            if self.is_active {
                                return false;
                            }
                            self.is_active = true;
                            self.app_state_agent.send(to_send.to_owned());
                            self.app_drawer_agent
                                .send(AppDrawerRequest::ToggleTopDrawer(0));
                            self.app_drawer_agent
                                .send(AppDrawerRequest::ToggleRightDrawer(0));
                            self.app_drawer_agent
                                .send(AppDrawerRequest::ToggleBottomDrawer(0));
                            self.app_drawer_agent
                                .send(AppDrawerRequest::ToggleLeftDrawer(0));
                            return true;
                        }
                        if self.is_active {
                            self.is_active = false;
                            return true;
                        }
                        return false;
                    }
                }
            }
            AppStateReceiverMessage::None => (),
        }
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        let classes = &mut Classes::new();
        classes.push("navlink");

        if !props.class.is_empty() {
            classes.push(&props.class);
        }

        if self.is_active {
            classes.push("active".to_owned());
        }

        let onclick = {
            let path = props.to.to_owned();
            ctx.link().callback(move |_| {
                AppStateReceiverMessage::AppStateMessage(AppStateRequest::PathUpdate(
                    path.to_string(),
                ))
            })
        };

        html! {
            <a href={props.to.to_string()} class={classes.clone()} onclick={onclick}>
                { for props.children.iter() }
            </a>
        }
    }
}
