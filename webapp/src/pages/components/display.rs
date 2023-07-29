use crate::prelude::*;

/// App home page
pub(crate) fn page_components_display(_contexts: Contexts) -> Html {
    set_title("Display Components");
    html! {
        <>
            <MarkdownContent href="/d/en-US/components/display.md" />
            <NextPage url="/about" />
        </>
    }
}
