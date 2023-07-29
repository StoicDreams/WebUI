use crate::prelude::*;

/// Starter page for privacy policy.
pub fn starter_page_privacy(_contexts: Contexts) -> Html {
    set_title(format!("{} Privacy Policy", get_app_name()).as_str());
    let tags = get_markdown_tags();
    html! {
        <>
            <MarkdownContent href="https://cdn.myfi.ws/d/en-US/privacy.md" {tags} />
        </>
    }
}
