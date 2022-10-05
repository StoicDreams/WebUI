use webui::Paper;
use yew::{function_component, html};

/// App page body component - page specific content is rendered here
#[function_component(PageHome)]
pub(crate) fn app_body() -> Html {
    html! {
        <>
            <Paper class="d-flex flex-row flex-wrapreverse">
                <Paper class="flex-grow">
                    <p>{"Welcome to the Web UI home page. We are super duper excited to be sharing this tool with you, as well as our journey into the world of Rust development."}</p>
                    <p>{"Stay tuned for more updates coming very soon"}</p>
                </Paper>
                <iframe class="discord-widget ml-a" src="https://discord.com/widget?id=972856291909332993&theme=dark" width="350" height="500" allowtransparency="true" frameborder="0" sandbox="allow-popups allow-popups-to-escape-sandbox allow-same-origin allow-scripts"></iframe>
            </Paper>
        </>
    }
}
