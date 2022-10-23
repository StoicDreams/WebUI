use crate::{function_component, html, Children, Classes, Properties, Theme};

/// Properties for Quote component
#[derive(Properties, PartialEq)]
pub struct QuoteProps {
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
    pub cite: String,
    #[prop_or_default]
    pub color: Theme,
}

/// Component for displaying a blockquote
///
#[function_component(Quote)]
pub fn quote(props: &QuoteProps) -> Html {
    let classes = &mut Classes::new();
    classes.push(format!("quote theme-dark highlight-{}", props.color));

    if props.elevation > 0 {
        classes.push(format!("elevation-{}", props.elevation));
    }

    if !props.class.is_empty() {
        classes.push(&props.class);
    }

    html! {
        <blockquote class={classes.to_string()} style={props.style.to_owned()}>
            { for props.children.iter() }
            if !props.cite.is_empty() {
                <cite>{props.cite.to_string()}</cite>
            }
        </blockquote>
    }
}
