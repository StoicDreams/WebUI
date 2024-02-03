use crate::prelude::*;

/// Component to display an icon
///
/// Expecting a Font Awesome or other CSS classes.
#[derive(Properties, PartialEq)]
pub struct IconOptions {
    /// Font Awesome or any other css classes for displaying icons
    pub icon: String,
    #[prop_or_default]
    pub elevation: u8,
    #[prop_or(Theme::None)]
    pub color: Theme,
    #[prop_or_default]
    pub title: String,
    #[prop_or_default]
    pub style: String,
    #[prop_or_default]
    pub class: String,
}

impl Default for IconOptions {
    fn default() -> Self {
        Self {
            icon: Default::default(),
            elevation: Default::default(),
            color: Default::default(),
            title: Default::default(),
            style: Default::default(),
            class: Default::default(),
        }
    }
}

#[function_component(Icon)]
pub fn icon(props: &IconOptions) -> Html {
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

    html! {
        <div class={classes.to_owned()} title={props.title.to_string()} aria-label={props.title.to_string()}>
            <i class={props.icon.to_owned()} />
        </div>
    }
}
