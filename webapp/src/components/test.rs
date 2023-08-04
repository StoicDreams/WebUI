use crate::prelude::*;

pub fn render_test(_contexts: Contexts) -> Html {
    html! {
        <Test />
    }
}

#[function_component(Test)]
pub fn test() -> Html {
    html! {
        <Paper class="pa-1">
            {"This is content from a Test component!"}
        </Paper>
    }
}
