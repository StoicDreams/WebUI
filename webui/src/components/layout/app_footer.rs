use crate::{AppConfig, Paper};
use yew::{function_component, html, use_context};

/// App footer component
#[function_component(AppFooter)]
pub(crate) fn app_footer() -> Html {
    let app_config = use_context::<AppConfig>().expect("no app config found");

    html! {
        <footer>
            <Paper class="grow" />
            <Paper>
                {format!("© {} {} All Rights Reserved", "2022", app_config.company_name)}
            </Paper>
            <Paper class="grow" />
            if !app_config.hide_powered_by {
                <Paper>
                    <sup>{"Powered by "}</sup>
                    <a href="https://webui.stoicdreams.com">{"Web UI"}</a>
                </Paper>
            }
        </footer>
    }
}
