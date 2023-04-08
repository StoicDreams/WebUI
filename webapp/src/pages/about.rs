use crate::prelude::*;

/// App page body component - page specific content is rendered here
pub(crate) fn page_about() -> Html {
    set_title(format!("About {}", get_company_singular()).as_str());
    html! {
        <>
            <MarkdownContent href="/d/en-US/about.md" />
            <NextPage url="/" />
        </>
    }
}
