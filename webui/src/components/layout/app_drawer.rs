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
    click_handler: ClickHandler,
}

/// Clonable struct to pass needed methods and data to click events
#[derive(Clone)]
struct ClickHandler {
    drawer: Direction,
}

impl ClickHandler {
    fn get_message(self: &Self) -> AppDrawerReceiverMessage {
        match self.drawer {
            Direction::Top => {
                AppDrawerReceiverMessage::AppDrawerMessage(AppDrawerRequest::ToggleTopDrawer(None))
            }
            Direction::Right => AppDrawerReceiverMessage::AppDrawerMessage(
                AppDrawerRequest::ToggleRightDrawer(None),
            ),
            Direction::Bottom => AppDrawerReceiverMessage::AppDrawerMessage(
                AppDrawerRequest::ToggleBottomDrawer(None),
            ),
            Direction::Left => {
                AppDrawerReceiverMessage::AppDrawerMessage(AppDrawerRequest::ToggleLeftDrawer(None))
            }
        }
    }
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
        let link = ctx.link();
        let test: &crate::html::Scope<AppDrawer> = link;
        Self {
            app_drawer_agent: AppDrawerAgent::bridge(
                ctx.link()
                    .callback(AppDrawerReceiverMessage::AppDrawerMessage),
            ),
            is_open: false,
            content: AppDrawerOptions::new("Loading...".to_owned(), || html! {}).build(),
            click_handler: ClickHandler {
                drawer: ctx.props().drawer.to_owned(),
            },
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
        let content = self.content.get_display();
        let show_header = !self.content.hide_header;
        let show_footer = !self.content.hide_footer;
        let show_close_x = !self.content.hide_close_x;
        let show_close = !self.content.hide_cancel;
        let drawer = ctx.props().drawer.to_owned();
        let cancel_button_display = "Cancel";
        let show_confirm = match self.content.on_confirm {
            Some(_) => true,
            None => false,
        };
        let confirm_display = self.content.confirm_display.to_owned();
        let confirm_onclick = self.content.get_on_confirm();

        let cover_click = self.click_handler.to_owned();
        let close_x_click = self.click_handler.to_owned();
        let close_click = self.click_handler.to_owned();
        let confirm_click = self.click_handler.to_owned();

        html! {
            <aside class={class}>
                <div class="drawer-placement">
                    <div class="page-cover" onclick={ctx.link().callback(move |_|cover_click.get_message())}>
                    </div>
                    <div class="drawer-content elevation-20">
                        {if show_header {
                            html! {
                                <header>
                                    {title_standard!(
                                        html!{
                                            <>
                                                <Paper>{"Give us your feedback!"}</Paper>
                                                <span class="flex-grow" />
                                            </>
                                        }
                                    )}
                                    <span class="flex-grow" />
                                    {if show_close_x {
                                        html! {
                                            <button class="btn theme-danger mr-1 pt-1 bt-1 pl-3 pr-3" onclick={ctx.link().callback(move |_|close_x_click.get_message())}>
                                                <i class="fa-solid fa-times" />
                                            </button>
                                        }
                                    } else {html!{}}}
                                </header>
                            }
                        }else{html!{}}}
                        <Paper class="flex-grow">
                            {content()}
                        </Paper>
                        {if show_footer {
                            html! {
                                <footer class="pa-1 d-flex flex-row">
                                    {if show_close {
                                        html! {
                                            <Button class="btn theme-warning" onclick={ctx.link().callback(move |_|close_click.get_message())}>
                                                {cancel_button_display}
                                            </Button>
                                        }
                                    } else {empty_html()}}
                                    {if show_confirm {
                                        html! {
                                            <>
                                                <span class="flex-grow" />
                                                <Button class="btn theme-success" onclick={ctx.link().callback(move |_|{
                                                    if !confirm_onclick() {
                                                        return AppDrawerReceiverMessage::None;
                                                    }
                                                    confirm_click.get_message()
                                                })}>
                                                    {confirm_display}
                                                </Button>
                                            </>
                                        }
                                    } else {empty_html()}}
                                </footer>
                            }
                        } else { html! {} }}
                    </div>
                </div>
            </aside>
        }
    }
}
