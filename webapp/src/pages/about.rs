use crate::*;
use webui::*;

/// App page body component - page specific content is rendered here
pub(crate) fn page_about() -> Html {
    set_title(format!("About {}", COMPANY_SINGULAR));
    html! {
        <>
            {title_secondary!(html!{"About Stoic Dreams"})}
            <Paper class="pa-3">
                {paragraphs!(
                    "Stoic Dreams is a software development studio with a focus on delivering tools and best practices to make software development easier and faster with the highest quality possible."
                )}
            </Paper>
        </>
    }
}
