use yew::{function_component, html, Properties, Children, Classes};

/// Properties for Paper component
#[derive(Properties, PartialEq)]
pub struct PaperProps {
    #[prop_or_default]
    pub elevation: u8,
    pub children: Children,
}

/// Common container component
#[function_component(Paper)]
pub fn paper(props: &PaperProps) -> Html {
    let classes = &mut Classes::new();
    classes.push("paper");

    if props.elevation > 0 {
        classes.push(format!("elevation-{}", props.elevation));
    }

    html! {
        <section class={classes.clone()}>
            { for props.children.iter() }
        </section>
    }
}
