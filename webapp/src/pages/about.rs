use webui::Paper;
use yew::{html, Html};

/// App page body component - page specific content is rendered here
pub(crate) fn page_about() -> Html {
    html! {
        <Paper>
            {"Welcome to the about page."}
        </Paper>
    }
}
