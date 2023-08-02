use crate::prelude::*;

#[derive(Properties, PartialEq)]
pub struct AlertProps {
    #[prop_or_default]
    pub elevation: u8,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: String,
    #[prop_or_default]
    pub style: String,
    #[prop_or(Theme::Title)]
    pub color: Theme,
}

#[function_component(Alert)]
pub fn alert(props: &AlertProps) -> Html {
    let classes = &mut Classes::new();
    classes.push(format!("alert {}", props.color));

    if props.elevation > 0 {
        classes.push(format!("elevation-{}", props.elevation));
    }

    if !props.class.is_empty() {
        classes.push(&props.class);
    } else {
        classes.push("pa-2");
    }

    html! {
        <Paper class={classes.to_string()} style={props.style.to_owned()}>
            { for props.children.iter() }
        </Paper>
    }
}
