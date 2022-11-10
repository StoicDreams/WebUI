use std::collections::HashMap;

use crate::*;

/// App page body component - page specific content is rendered here
pub(crate) fn page_terms() -> Html {
    set_title(format!("{} Terms & Conditions", COMPANY_SINGULAR).as_str());
    let mut tags = HashMap::<String, String>::new();
    tags.insert(
        String::from("COMPANY_SINGULAR"),
        String::from(COMPANY_SINGULAR),
    );
    tags.insert(String::from("COMPANY_PLURAL"), String::from(COMPANY_PLURAL));
    html! {
        <>
            <MarkdownContent href="/d/en-US/terms.md" {tags} />
            <NextPage url="/privacy" />
        </>
    }
}
