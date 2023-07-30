use crate::prelude::*;

/// App page body component - page specific content is rendered here
pub(crate) fn page_about(_contexts: Contexts) -> Html {
    set_title(format!("About {}", get_company_singular()).as_str());
    html! {
        <>
            <MarkdownContent href="https://cdn.myfi.ws/d/en-US/about_stoic_dreams.md" />
            <NextPageButton url="/" />
        </>
    }
}
