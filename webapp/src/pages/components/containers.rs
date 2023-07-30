use crate::prelude::*;

/// App home page
pub(crate) fn page_components_containers(_contexts: Contexts) -> Html {
    set_title("Paper components");
    html! {
        <>
            <MarkdownContent href="/d/en-US/components/containers.md" />
            <NextPageButton url="/components/display" />
        </>
    }
}
