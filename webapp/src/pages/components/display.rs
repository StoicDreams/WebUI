use webui::*;

/// App home page
pub(crate) fn page_components_display() -> Html {
    set_title("Display Components".to_string());
    html! {
        <>
            <MarkdownContent href="/d/en-US/components/display.md" />
        </>
    }
}
