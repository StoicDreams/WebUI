use crate::*;

pub(crate) fn page_classes_helpers() -> Html {
    set_title("Web UI Helper Classes".to_string());
    html! {
        <>
            <MarkdownContent href="/d/en-US/classes/helpers.md" />
            <JasperLink display="Content on this website was created with the help of Jasper.ai." />
        </>
    }
}
