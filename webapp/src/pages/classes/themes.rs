use webui::*;

/// App home page
pub(crate) fn page_classes_themes() -> Html {
    set_title("Web UI Demo & Documentation".to_string());
    html! {
        <>
            {title_primary!("Theme Classes")}
            <Paper class={CLASSES_PAGE_SECTION} elevation={ELEVATION_STANDARD}>
                {paragraphs!(
                    "Web UI groups various color settings into theme classes.",
                    "Each theme consists of a background color and foreground color (e.g. text).",
                    "More information coming soon."
                )}
            </Paper>
        </>
    }
}
