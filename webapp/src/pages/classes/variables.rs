use crate::*;

pub(crate) fn page_classes_variables() -> Html {
    set_title("Rust Class Variable Helpers".to_string());
    html! {
        <>
            <MarkdownContent href="/d/en-US/classes/variables.md" />
            <JasperLink display="Content on this website was created with the help of Jasper.ai." />
        </>
    }
}
