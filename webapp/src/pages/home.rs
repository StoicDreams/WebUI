use crate::*;

/// App home page
pub(crate) fn page_home() -> Html {
    set_title("Web UI Demo & Documentation");
    html! {
        <>
            <MarkdownContent href="/d/en-US/home.md" />
            <NextPage url="/classes/helpers" />
        </>
    }
}
