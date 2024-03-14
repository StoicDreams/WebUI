use crate::prelude::*;

/// Starter page for the terms and conditions.
pub fn starter_page_terms(_contexts: &Contexts) -> Html {
    set_title(format!("{} Terms & Conditions", get_app_name()).as_str());
    let tags = get_markdown_tags();
    html! {
        <>
            <MarkdownContent href="https://cdn.myfi.ws/d/en-US/terms.md" {tags} />
            <NextPageButton url="/privacy" />
        </>
    }
}
