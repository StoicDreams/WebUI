use crate::{AppConfig, Paper};
use yew::{function_component, html, use_context};

/// App header component
#[function_component(AppHeader)]
pub(crate) fn app_header() -> Html {
    let app_config = use_context::<AppConfig>().expect("no app config found");

    html! {
        <header>
            <Paper>{ app_config.app_name }</Paper>
        </header>
    }
}
