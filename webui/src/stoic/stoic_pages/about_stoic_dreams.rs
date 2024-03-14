use std::collections::HashMap;

use crate::prelude::*;

/// App page body component - page specific content is rendered here
pub fn page_about_stoic_dreams(_contexts: &Contexts) -> Html {
    set_title("About Stoic Dreams");
    let tags = get_markdown_tags();
    let tag_copy = tags.clone();
    html! {
        <>
            <MarkdownContent href="/d/en-US/about.md" {tags} />
            {include_stoic_dreams_section(tag_copy)}
            <NextPageButton url="/" />
        </>
    }
}

fn include_stoic_dreams_section(tags: HashMap<String, String>) -> Html {
    if get_company_singular() == "Stoic Dreams" {
        html! {
            <MarkdownContent href="https://cdn.myfi.ws/d/en-US/about_stoic_dreams.md" tags={tags} />
        }
    } else {
        html! {}
    }
}
