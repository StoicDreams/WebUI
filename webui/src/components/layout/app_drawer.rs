use crate::*;

/// Properties for app drawer components
#[derive(Properties, PartialEq)]
pub(crate) struct AppDrawerProps {
    #[prop_or_default]
    pub class: Option<String>,
    pub drawer: Direction,
}

#[derive(Default, Clone, PartialEq, Debug)]
pub(crate) struct AppDrawerState {
    pub is_open: bool,
    pub content: Option<AppDrawerOptions>,
}

pub(crate) struct AppDrawer {
    app_drawer_agent: Box<dyn Bridge<AppDrawerAgent>>,
    is_open: bool,
    content: AppDrawerOptions,
}

impl AppDrawer {
    fn toggle_state(&mut self, content_ref: Option<AppDrawerOptions>) {
        match content_ref {
            Some(options) => {
                self.is_open = options.display_ref > 0 && !self.is_open;
                if options.display_ref > 0 {
                    self.content = options.clone();
                }
            }
            None => {
                self.is_open = false;
            }
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
            content: AppDrawerOptions::new("Loading...".to_owned(), || html! {}).build(),
        }
    }

    fn update(&mut self, ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        let is_open = self.is_open.clone();
        match msg {
            AppDrawerReceiverMessage::AppDrawerMessage(message) => {
                match message {
                    AppDrawerRequest::ToggleTopDrawer(fnval) => {
                        if ctx.props().drawer != Direction::Top {
                            return false;
                        }
                        self.toggle_state(fnval);
                    }
                    AppDrawerRequest::ToggleRightDrawer(fnval) => {
                        if ctx.props().drawer != Direction::Right {
                            return false;
                        }
                        self.toggle_state(fnval);
                    }
                    AppDrawerRequest::ToggleBottomDrawer(fnval) => {
                        if ctx.props().drawer != Direction::Bottom {
                            return false;
                        }
                        self.toggle_state(fnval);
                    }
                    AppDrawerRequest::ToggleLeftDrawer(fnval) => {
                        if ctx.props().drawer != Direction::Left {
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
            "app-drawer {} {} {}",
            props.drawer,
            props.class.to_owned().unwrap_or_default(),
            if self.is_open { "open" } else { "closed" }
        );
        let content = self.content.clone().get_display();
        let show_title = !self.content.hide_title;
        let show_close_x = !self.content.hide_close_x;
        let drawer_cover = ctx.props().drawer.to_owned();
        let drawer_placement = ctx.props().drawer.to_owned();
        let drawer_close_x = ctx.props().drawer.to_owned();
        html! {
            <aside class={class}>
                <div class="page-cover" onclick={ctx.link().callback(move |_|
                    {
                        match drawer_cover {
                            Direction::Top => return AppDrawerReceiverMessage::AppDrawerMessage(AppDrawerRequest::ToggleTopDrawer(None)),
                            Direction::Right => return AppDrawerReceiverMessage::AppDrawerMessage(AppDrawerRequest::ToggleRightDrawer(None)),
                            Direction::Bottom => return AppDrawerReceiverMessage::AppDrawerMessage(AppDrawerRequest::ToggleBottomDrawer(None)),
                            Direction::Left => AppDrawerReceiverMessage::AppDrawerMessage(AppDrawerRequest::ToggleLeftDrawer(None)),
                        }
                    }
                )}>
                </div>
                <div class="drawer-placement" onclick={ctx.link().callback(move |_|
                    {
                        match drawer_placement {
                            Direction::Top => return AppDrawerReceiverMessage::AppDrawerMessage(AppDrawerRequest::ToggleTopDrawer(None)),
                            Direction::Right => return AppDrawerReceiverMessage::AppDrawerMessage(AppDrawerRequest::ToggleRightDrawer(None)),
                            Direction::Bottom => return AppDrawerReceiverMessage::AppDrawerMessage(AppDrawerRequest::ToggleBottomDrawer(None)),
                            Direction::Left => AppDrawerReceiverMessage::AppDrawerMessage(AppDrawerRequest::ToggleLeftDrawer(None)),
                        }
                    }
                )}>
                    <div class="drawer-content">

                        {if show_title {
                            {title_standard!(
                                html!{
                                    <>
                                        <Paper>{"Give us your feedback!"}</Paper>
                                        <span class="flex-grow" />
                                        {if show_close_x {
                                            html! {
                                                <button class="btn theme-danger mr-1 pt-1 bt-1 pl-3 pr-3" onclick={ctx.link().callback(move |_|
                                                    {
                                                        match drawer_close_x {
                                                            Direction::Top => return AppDrawerReceiverMessage::AppDrawerMessage(AppDrawerRequest::ToggleTopDrawer(None)),
                                                            Direction::Right => return AppDrawerReceiverMessage::AppDrawerMessage(AppDrawerRequest::ToggleRightDrawer(None)),
                                                            Direction::Bottom => return AppDrawerReceiverMessage::AppDrawerMessage(AppDrawerRequest::ToggleBottomDrawer(None)),
                                                            Direction::Left => AppDrawerReceiverMessage::AppDrawerMessage(AppDrawerRequest::ToggleLeftDrawer(None)),
                                                        }
                                                    }
                                                )}>
                                                    <i class="fa-solid fa-times" />
                                                </button>
                                            }
                                        } else {html!{}}}
                                    </>
                                }
                            )}
                        }else{html!{}}}

                        {content()}
                    </div>
                </div>
            </aside>
        }
    }
}
