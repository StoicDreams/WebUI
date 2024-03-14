use crate::prelude::*;

/// App home page
pub(crate) fn page_home(_contexts: &Contexts) -> Html {
    set_title("Web UI Demo & Documentation");
    html! {
        <>
            <MarkdownContent href="/d/en-US/home.md" />
            <NextPageButton url="/classes/helpers" />
        </>
    }
}
