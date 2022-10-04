use yew::{function_component, html, use_context};
use crate::AppConfig;

/// App footer component
#[function_component(AppFooter)]
pub(crate) fn app_footer() -> Html {
    let app_config = use_context::<AppConfig>().expect("no app config found");

    html! {
        <footer>
            <span>
                {format!("© {} {} All Rights Reserved", "2022", app_config.company_name)}
            </span>
        </footer>
    }
}
