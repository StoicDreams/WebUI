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
    pub icon: Option<String>,
    #[prop_or_default]
    pub image: Option<String>,
    #[prop_or_default]
    pub style: String,
}

/// Component for displaying an icon or image
///
/// Image or icon typically should have square dimensions.
///
/// Basic example displaying Font Awesome icon
/// ```rust
/// use webui::*;
///
/// fn page() -> Html {
/// 	html! {
/// 		<Image icon="fa-solid fa-acorn" alt="Acorn Icon" />
/// 	}
/// }
/// ```
///
/// Add classes
/// ```rust
/// use webui::*;
///
/// fn page() -> Html {
/// 	html! {
/// 		<Image image="Logo.svg" alt="" class="d-flex flex-column" />
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
/// 		<Image image="Logo.svg" elevation={10} />
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
            {match props.icon.to_owned() {
                Some(class) => html! {
                    <i {class} alt={props.alt.to_string()} title={props.alt.to_string()} />
                },
                None => {
                    {match props.image.to_owned() {
                        Some(src) => html! {
                            <img {src} alt={props.alt.to_string()} title={props.alt.to_string()} />
                        },
                        None => html! {}
                    }}
                }
            }}
        </Paper>
    }
}
