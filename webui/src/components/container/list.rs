use crate::{function_component, html, Children, Classes, Html, Properties};

/// Properties for List component
#[derive(Properties, PartialEq)]
pub struct ListProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub elevation: u8,
    #[prop_or_default]
    pub class: String,
    #[prop_or_default]
    pub alt: String,
    #[prop_or_default]
    pub style: String,
    #[prop_or_default]
    pub is_ordered: bool,
}

/// Component for displaying a list of items
///
#[function_component(List)]
pub fn list(props: &ListProps) -> Html {
    let classes = &mut Classes::new();
    classes.push("list");

    if props.elevation > 0 {
        classes.push(format!("elevation-{}", props.elevation));
    }

    if !props.class.is_empty() {
        classes.push(&props.class);
    }

    if props.is_ordered {
        html! {
            <ol class={classes.to_string()} style={props.style.to_owned()}>
                { for props.children.iter() }
            </ol>
        }
    } else {
        html! {
            <ul class={classes.to_string()} style={props.style.to_owned()}>
                { for props.children.iter() }
            </ul>
        }
    }
}
