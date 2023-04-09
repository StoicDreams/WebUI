use crate::prelude::*;

/// Starter page for under construction.
pub fn starter_page_under_construction() -> Html {
    set_title(format!("{} Under Construction", get_app_name()).as_str());
    let tags = get_markdown_tags();
    html! {
        <>
            <MarkdownContent href="/d/en-US/under_construction.md" {tags} />
        </>
    }
}
