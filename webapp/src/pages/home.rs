use crate::*;

/// App home page
pub(crate) fn page_home() -> Html {
    set_title("Web UI Demo & Documentation".to_string());
    html! {
        <>
            <MarkdownContent href="/d/en-US/home.md" />
            <JasperLink display="Content on this website was created with the help of Jasper.ai." />
        </>
    }
}
