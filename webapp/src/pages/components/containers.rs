use crate::*;

/// App home page
pub(crate) fn page_components_containers() -> Html {
    set_title("Paper components");
    html! {
        <>
            <MarkdownContent href="/d/en-US/components/containers.md" />
            <Next url="/components/display" />
        </>
    }
}
