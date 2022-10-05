use crate::AppConfig;
use yew::{function_component, html, use_context};

/// App page body component - page specific content is rendered here
#[function_component(AppBody)]
pub(crate) fn app_body() -> Html {
    let app_config = use_context::<AppConfig>().expect("no app config found");

    html! {
        <main>
            {(app_config.body_html)()}
        </main>
    }
}
