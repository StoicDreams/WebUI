use super::app_body::AppBody;
use super::app_contexts::AppContexts;
use super::app_drawer::AppDrawer;
use super::app_footer::AppFooter;
use super::app_header::AppHeader;
use crate::data_types::app_config::AppConfig;
use crate::prelude::*;

/// Properties for AppConfig component
#[derive(Properties, PartialEq)]
pub(crate) struct AppProps {
    pub config: AppConfig,
    pub state_keys: Vec<String>,
}

/// Inner process for starting website
pub(crate) fn start_webui_app(app_config: AppConfig, state_keys: Vec<String>) {
    let props = AppProps {
        config: app_config,
        state_keys,
    };
    yew::Renderer::<App>::with_props(props).render();
}
struct App {}

impl Component for App {
    type Message = ();
    type Properties = AppProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        html! {
            <div id="app" class="page transition out">
                <AppContexts
                    config={props.config.clone()}
                    state_keys={props.state_keys.clone()}
                    >
                    <AppHeader />
                    <AppBody />
                    <AppFooter />
                    <AppDrawer drawer={Direction::Top} />
                    <AppDrawer drawer={Direction::Right} />
                    <AppDrawer drawer={Direction::Bottom} />
                    <AppDrawer drawer={Direction::Left} />
                    <crate::loaders::Loaders />
                </AppContexts>
            </div>
        }
    }
}
