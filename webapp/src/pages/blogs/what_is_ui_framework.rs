use crate::*;

/// App home page
pub(crate) fn page_blogs_what_is_ui_framework() -> Html {
    set_title("What is a Website UI Framework?".to_string());
    html! {
        <>
            <MarkdownContent href="/d/en-US/blogs/what_is_ui_framework.md" />
            <JasperLink display="Content on this website was created with the help of Jasper.ai." />
        </>
    }
}
