use crate::prelude::*;

/// Properties for Button components
#[derive(Properties, PartialEq)]
pub struct ButtonProps {
    #[prop_or_default]
    pub elevation: u8,
    #[prop_or_default]
    pub children: Children,
    #[prop_or(Theme::None)]
    pub color: Theme,
    #[prop_or_default]
    pub class: String,
    #[prop_or_default]
    pub title: String,
    #[prop_or_default]
    pub style: String,
    //#[prop_or_default]
    //pub onclick: fn(ev:MouseEvent),
    #[prop_or_default]
    pub onclick: Option<Callback<MouseEvent>>,
    #[prop_or_default]
    pub disabled: bool,
}

/// Display button
#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    let classes = &mut Classes::new();

    if !props.class.is_empty() {
        classes.push(&props.class);
    } else {
        classes.push("btn");
    }

    if props.elevation > 0 {
        classes.push(format!("elevation-{}", props.elevation));
    }

    if props.color != Theme::None {
        classes.push(format!("{}", props.color));
    }

    let onclick = match props.onclick.to_owned() {
        Some(callback) => callback,
        None => Callback::default(),
    };
    if props.disabled {
        classes.push("disabled");
        return html! {
            <button type="button" class={classes.to_owned()} title={props.title.to_string()} aria-label={props.title.to_string()} disabled={true}>
                {for props.children.iter()}
            </button>
        };
    }
    html! {
        <button type="button" class={classes.to_owned()} title={props.title.to_string()} aria-label={props.title.to_string()} onclick={onclick}>
            {for props.children.iter()}
        </button>
    }
}
