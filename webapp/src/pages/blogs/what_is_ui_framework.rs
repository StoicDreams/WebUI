use crate::prelude::*;

/// App home page
pub(crate) fn page_blogs_what_is_ui_framework() -> Html {
    set_title("What is a Website UI Framework?");
    html! {
        <>
            <MarkdownContent href="/d/en-US/blogs/what_is_ui_framework.md" />
            <NextPage url="/" />
        </>
    }
}
