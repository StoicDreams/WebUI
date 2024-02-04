use crate::prelude::*;

/// Properties for Paper component
#[derive(Properties, PartialEq)]
pub struct PaperProps {
    #[prop_or_default]
    pub elevation: u8,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: String,
    #[prop_or_default]
    pub style: String,
    #[prop_or_default]
    pub id: String,
    #[prop_or_default]
    pub title: String,
}

/// Common container component
///
/// Basic example
/// ```rust
/// use webui::prelude::*;
///
/// fn page(contexts: Contexts) -> Html {
///     html! {
///         <Paper>{"Your child content here"}</Paper>
///     }
/// }
/// ```
///
/// Add classes
/// ```rust
/// use webui::prelude::*;
///
/// fn page(contexts: Contexts) -> Html {
///     html! {
///         <Paper class="d-flex flex-column">{"Your child content here"}</Paper>
///     }
/// }
/// ```
///
/// Apply elevetation
///
/// Elevation applies a box shadow to the Paper component.
/// Valid ranges range from 0 ro 25.
/// ```rust
/// use webui::prelude::*;
///
/// fn page(contexts: Contexts) -> Html {
///     html! {
///         <Paper elevation={10}>{"Your child content here"}</Paper>
///     }
/// }
/// ```
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

    let class = classes.clone();
    let style = if props.style.is_empty() {
        None
    } else {
        Some(props.style.to_owned())
    };
    let id = if props.id.is_empty() {
        None
    } else {
        Some(props.id.to_owned())
    };
    let title = props.title.to_owned();

    html! {
        <section {id} {class} {style} {title}>
            { for props.children.iter() }
        </section>
    }
}
