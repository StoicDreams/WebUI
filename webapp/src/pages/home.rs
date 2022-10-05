use webui::Paper;
use yew::{function_component, html};

/// App page body component - page specific content is rendered here
#[function_component(PageHome)]
pub(crate) fn app_body() -> Html {
    html! {
        <Paper>
            {"Welcome to the home page."}
        </Paper>
    }
}
