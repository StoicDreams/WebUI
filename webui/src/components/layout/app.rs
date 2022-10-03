use super::app_body::AppBody;
use super::app_drawer_bottom::AppDrawerBottom;
use super::app_drawer_left::AppDrawerLeft;
use super::app_drawer_right::AppDrawerRight;
use super::app_drawer_top::AppDrawerTop;
use super::app_footer::AppFooter;
use super::app_header::AppHeader;
use crate::data_types::app_config::AppConfig;
use yew::{function_component, html, use_state, ContextProvider, Properties};

#[derive(Properties, PartialEq)]
pub struct AppProps {
    pub config: AppConfig,
}

#[function_component(App)]
pub(crate) fn app(props: &AppProps) -> Html {
    let ctx = use_state(|| props.config.clone());

    html! {
        <div id="app">
            <ContextProvider<AppConfig> context={(*ctx).clone()}>
                <AppHeader />
                <AppBody />
                <AppFooter />
                <AppDrawerTop />
                <AppDrawerRight />
                <AppDrawerBottom />
                <AppDrawerLeft />
            </ContextProvider<AppConfig>>
        </div>
    }
}
