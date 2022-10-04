use yew::{function_component, html};

/// App page body component - page specific content is rendered here
#[function_component(AppBody)]
pub(crate) fn app_body() -> Html {
    html! {
        <main>
            { "Coming Soon!" }
        </main>
    }
}
