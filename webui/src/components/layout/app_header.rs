use crate::AppConfig;
use yew::{function_component, html, use_context};

#[function_component(AppHeader)]
pub(crate) fn app_header() -> Html {
    let app_config = use_context::<AppConfig>().expect("no app config found");

    html! {
        <header>
            { app_config.app_name }
        </header>
    }
}
