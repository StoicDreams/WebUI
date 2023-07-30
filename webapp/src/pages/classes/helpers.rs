use crate::prelude::*;

pub(crate) fn page_classes_helpers(_contexts: Contexts) -> Html {
    set_title("Web UI Helper Classes");
    html! {
        <>
            <MarkdownContent href="/d/en-US/classes/helpers.md" />
            <NextPageButton url="/classes/themes" />
        </>
    }
}
