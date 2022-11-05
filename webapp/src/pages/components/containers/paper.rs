use webui::*;

/// App home page
pub(crate) fn page_components_containers_paper() -> Html {
    set_title("Paper components".to_string());
    html! {
        <>
            <MarkdownContent href="/d/en-US/components/containers/paper.md" />
        </>
    }
}
