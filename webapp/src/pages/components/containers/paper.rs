use webui::*;

/// App home page
pub(crate) fn page_components_containers_paper() -> Html {
    set_title("Paper components".to_string());
    html! {
        <>
            {title_primary!("Paper components")}
            <Paper class={CLASSES_PAGE_SECTION} elevation={ELEVATION_STANDARD}>
                {paragraphs!(
                    "Paper components are the base container for which to hold other elements or data in.",
                    "More information coming soon."
                )}
            </Paper>
        </>
    }
}
