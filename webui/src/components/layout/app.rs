use super::app_body::AppBody;
use super::app_contexts::AppContexts;
use super::app_drawer::AppDrawer;
use super::app_footer::AppFooter;
use super::app_header::AppHeader;
use crate::data_types::app_config::AppConfig;
use yew::prelude::*;

/// Properties for AppConfig component
#[derive(Properties, PartialEq)]
pub(crate) struct AppProps {
    pub config: AppConfig,
}

/// Inner process for starting website
pub(crate) fn start_webui_app(app_config: AppConfig) {
    let props = AppProps { config: app_config };
    yew::start_app_with_props::<App>(props);
}
struct App;

impl Component for App {
    type Message = ();
    type Properties = AppProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        html! {
            <div id="app">
                <AppContexts app_config={props.config.clone()}>
                    <AppHeader />
                    <AppBody />
                    <AppFooter />
                    <AppDrawer class="top" />
                    <AppDrawer class="right" />
                    <AppDrawer class="bottom" />
                    <AppDrawer class="left" />
                </AppContexts>
            </div>
        }
    }
}
