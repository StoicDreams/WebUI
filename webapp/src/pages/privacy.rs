use std::collections::HashMap;

use crate::*;

/// App page body component - page specific content is rendered here
pub(crate) fn page_privacy() -> Html {
    set_title(format!("{} Privacy Policy", COMPANY_SINGULAR));
    let mut tags = HashMap::<String, String>::new();
    tags.insert(String::from("COMPANY_SINGULAR"), String::from(COMPANY_SINGULAR));
    tags.insert(String::from("APP_NAME"), String::from(APP_NAME));
    html! {
        <>
            <MarkdownContent href="/d/en-US/privacy.md" {tags} />
        </>
    }
}
