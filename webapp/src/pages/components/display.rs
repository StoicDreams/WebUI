use crate::*;

/// App home page
pub(crate) fn page_components_display() -> Html {
    set_title("Display Components");
    html! {
        <>
            <MarkdownContent href="/d/en-US/components/display.md" />
            <NextPage url="/about" />
        </>
    }
}
