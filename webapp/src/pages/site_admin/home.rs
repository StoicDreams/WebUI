use crate::prelude::*;

/// App page body component - page specific content is rendered here
pub(crate) fn page_site_admin_home(_contexts: &Contexts) -> Html {
    set_title(format!("Site Admin Home {}", get_company_singular()).as_str());
    html! {
        <>
            <MarkdownContent href="/d/en-US/sa/home.md" />
            <NextPageButton url="/" />
        </>
    }
}
