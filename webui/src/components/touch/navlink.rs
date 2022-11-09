use crate::*;

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

/// Navigation link component
///
/// Use this when you want to display a navigation button or link
/// Make sure the `to` property starts with a forward slash `/`
///
/// example to display a link
/// ```rust
/// use webui::*;
///
/// fn component() -> Html {
///     html! {
///         <NavLink to="/some-page">{"A Link"}</NavLink>
///     }
/// }
/// ```
///
/// example to display a button
/// ```rust
/// use webui::*;
///
/// fn component() -> Html {
///     html! {
///         <NavLink to="/some-page">{"A Link"}</NavLink>
///     }
/// }
/// ```
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
            AppStateReceiverMessage::AppStateMessage(request) => match request {
                AppStateRequest::PathUpdate(path) => {
                    if ctx.props().to.to_lowercase() == path.to_lowercase() {
                        if self.is_active {
                            return false;
                        }
                        self.is_active = true;
                        self.app_drawer_agent
                            .send(AppDrawerRequest::ToggleTopDrawer(None));
                        self.app_drawer_agent
                            .send(AppDrawerRequest::ToggleRightDrawer(None));
                        self.app_drawer_agent
                            .send(AppDrawerRequest::ToggleBottomDrawer(None));
                        self.app_drawer_agent
                            .send(AppDrawerRequest::ToggleLeftDrawer(None));
                        return true;
                    }
                    if self.is_active {
                        self.is_active = false;
                        return true;
                    }
                    return false;
                }
            },
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
                push_state(&path);
                let mut app_state_agent = AppStateAgent::dispatcher();
                app_state_agent.send(AppStateRequest::PathUpdate(path.to_string()));
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
