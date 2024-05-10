use crate::{function_component, html, Classes, FaIcon, Html, Paper, Properties};

/// Properties for Avatar component
#[derive(Properties, PartialEq)]
pub struct AvatarProps {
    #[prop_or_default]
    pub elevation: u8,
    #[prop_or_default]
    pub class: String,
    #[prop_or_default]
    pub alt: String,
    #[prop_or_default]
    pub icon: Option<FaIcon>,
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
/// use webui::prelude::*;
///
/// fn page(contexts: &Contexts) -> Html {
///     html! {
///         <Avatar icon={FaIcon::solid("acorn")} alt="Acorn Icon" />
///     }
/// }
/// ```
///
/// Add classes
/// ```rust
/// use webui::prelude::*;
///
/// fn page(contexts: &Contexts) -> Html {
///     html! {
///         <Avatar image="Logo.svg" alt="" class="d-flex flex-column" />
///     }
/// }
/// ```
///
/// Apply elevetation
///
/// Elevation applies a box shadow to the Avatar component.
/// Valid ranges range from 0 ro 25.
/// ```rust
/// use webui::prelude::*;
///
/// fn page(contexts: &Contexts) -> Html {
///     html! {
///         <Avatar image="Logo.svg" elevation={10} />
///     }
/// }
/// ```
#[function_component(Avatar)]
pub fn avatar(props: &AvatarProps) -> Html {
    let classes = &mut Classes::new();
    classes.push("avatar");

    if props.elevation > 0 {
        classes.push(format!("elevation-{}", props.elevation));
    }

    if !props.class.is_empty() {
        classes.push(&props.class);
    }

    html! {
        <Paper class={classes.to_string()} style={props.style.to_owned()}>
            {match props.icon.to_owned() {
                Some(fa_icon) => html! {
                    {fa_icon.to_html()}
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
