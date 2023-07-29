use crate::prelude::*;

/// This page is used to display a page under construction.
pub(crate) fn page_under_construction(contexts: Contexts) -> Html {
    set_title("Web UI Under Construction Placeholder");
    let tags = get_markdown_tags();
    html! {
        <>
            <Quote Colo={Theme::Primary}>
                <p>{"This page showcases one of WebUI's starter pages."}</p>
            </Quote>
            <MarkdownContent href="https://cdn.myfi.ws/d/en-US/under_construction.md" {tags} />
            <NextPage url="/" />
        </>
    }
}
