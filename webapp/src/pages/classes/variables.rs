use crate::*;

pub(crate) fn page_classes_variables() -> Html {
    set_title("Rust Class Variable Helpers");
    html! {
        <>
            <MarkdownContent href="/d/en-US/classes/variables.md" />
            <Next url="/components/containers" />
        </>
    }
}
