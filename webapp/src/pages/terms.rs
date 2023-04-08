use std::collections::HashMap;

use crate::prelude::*;

/// App page body component - page specific content is rendered here
pub(crate) fn page_terms() -> Html {
    set_title(format!("{} Terms & Conditions", get_company_singular()).as_str());
    let mut tags = HashMap::<String, String>::new();
    tags.insert(
        String::from("COMPANY_SINGULAR"),
        String::from(get_company_singular()),
    );
    tags.insert(
        String::from("COMPANY_PLURAL"),
        String::from(get_company_plural()),
    );
    html! {
        <>
            <MarkdownContent href="/d/en-US/terms.md" {tags} />
            <NextPage url="/privacy" />
        </>
    }
}
