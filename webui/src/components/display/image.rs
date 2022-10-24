use crate::{function_component, html, Classes, Paper, Properties};

/// Properties for Image component
#[derive(Properties, PartialEq)]
pub struct ImageProps {
    #[prop_or_default]
    pub elevation: u8,
    #[prop_or_default]
    pub class: String,
    #[prop_or_default]
    pub alt: String,
    #[prop_or_default]
    pub src: String,
    #[prop_or_default]
    pub style: String,
}

/// Component for displaying an image
///
/// Basic example displaying image with classes
/// ```rust
/// use webui::*;
///
/// fn page() -> Html {
/// 	html! {
/// 		<Image src="Logo.svg" alt="" class="d-flex flex-column" />
/// 	}
/// }
/// ```
///
/// Apply elevetation
///
/// Elevation applies a box shadow to the Image component.
/// Valid ranges range from 0 ro 25.
/// ```rust
/// use webui::*;
///
/// fn page() -> Html {
/// 	html! {
/// 		<Image src="Logo.svg" elevation={10} />
/// 	}
/// }
/// ```
#[function_component(Image)]
pub fn image(props: &ImageProps) -> Html {
    let classes = &mut Classes::new();
    classes.push("image");

    if props.elevation > 0 {
        classes.push(format!("elevation-{}", props.elevation));
    }

    if !props.class.is_empty() {
        classes.push(&props.class);
    }

    html! {
        <Paper class={classes.to_string()} style={props.style.to_owned()}>
            <img src={props.src.to_string()} alt={props.alt.to_string()} title={props.alt.to_string()} />
        </Paper>
    }
}
