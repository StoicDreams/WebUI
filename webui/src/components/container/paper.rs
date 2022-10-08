use crate::{function_component, html, Children, Classes, Properties};

/// Properties for Paper component
#[derive(Properties, PartialEq)]
pub struct PaperProps {
    #[prop_or_default]
    pub elevation: u8,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: String,
}

/// Common container component
#[function_component(Paper)]
pub fn paper(props: &PaperProps) -> Html {
    let classes = &mut Classes::new();
    classes.push("paper");

    if props.elevation > 0 {
        classes.push(format!("elevation-{}", props.elevation));
    }

    if !props.class.is_empty() {
        classes.push(&props.class);
    }

    html! {
        <section class={classes.clone()}>
            { for props.children.iter() }
        </section>
    }
}
