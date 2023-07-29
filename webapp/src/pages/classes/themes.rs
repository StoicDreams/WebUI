use crate::prelude::*;

/// App home page
pub(crate) fn page_classes_themes(_contexts: Contexts) -> Html {
    set_title("Web UI Demo & Documentation");
    html! {
        <>
            <MarkdownContent href="/d/en-US/classes/themes.md" />
            <NextPage url="/classes/variables" />
        </>
    }
}
