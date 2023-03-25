use std::collections::HashMap;

use crate::*;

/// App page body component - page specific content is rendered here
pub(crate) fn page_privacy() -> Html {
    set_title(format!("{} Privacy Policy", get_company_singular()).as_str());
    let mut tags = HashMap::<String, String>::new();
    tags.insert(
        String::from("COMPANY_SINGULAR"),
        String::from(get_company_singular()),
    );
    tags.insert(String::from("APP_NAME"), String::from(get_app_name()));
    html! {
        <>
            <MarkdownContent href="/d/en-US/privacy.md" {tags} />
            <NextPage url="/terms" />
        </>
    }
}
