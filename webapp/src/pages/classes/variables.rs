use crate::prelude::*;

pub(crate) fn page_classes_variables(_contexts: Contexts) -> Html {
    set_title("Rust Class Variable Helpers");
    html! {
        <>
            <MarkdownContent href="/d/en-US/classes/variables.md" />
            <NextPageButton url="/components/containers" />
        </>
    }
}
