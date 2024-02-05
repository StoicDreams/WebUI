use crate::prelude::*;

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
    pub title: String,
    #[prop_or_default]
    pub src: String,
    #[prop_or_default]
    pub style: String,
}

/// Component for displaying an image
///
/// Basic example displaying image with classes
/// ```rust
/// use webui::prelude::*;
///
/// fn page(contexts: Contexts) -> Html {
///     html! {
///         <Image src="Logo.svg" alt="" class="d-flex flex-column" />
///     }
/// }
/// ```
///
/// Apply elevetation
///
/// Elevation applies a box shadow to the Image component.
/// Valid ranges range from 0 ro 25.
/// ```rust
/// use webui::prelude::*;
///
/// fn page(contexts: Contexts) -> Html {
///     html! {
///         <Image src="Logo.svg" elevation={10} />
///     }
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
    let is_svg = props.src.ends_with(".svg");
    html! {
        <Paper class={classes.to_string()} style={props.style.to_owned()} title={props.title.to_owned()}>
            {if is_svg {
                html!{<ImageSVG src={expand_url(&props.src)} alt={props.alt.to_string()} title={props.alt.to_string()} />}
            }else{
                html!{<ImageIMG src={props.src.to_string()} alt={props.alt.to_string()} title={props.alt.to_string()} />}
            }}
        </Paper>
    }
}

#[derive(Properties, PartialEq, Clone)]
struct UrlProp {
    src: AttrValue,
    alt: AttrValue,
    title: AttrValue,
}

#[function_component(ImageIMG)]
fn image_img(props: &UrlProp) -> Html {
    html! {<img src={props.src.to_string()} alt={props.alt.to_string()} title={props.alt.to_string()} />}
}

#[function_component(ImageSVG)]
fn image_svg(props: &UrlProp) -> Html {
    let contexts = use_context::<Contexts>().expect("Failed to load contexts");
    let image = use_state(|| String::default());
    if image.is_empty() {
        let src = props.src.clone();
        let image = image.clone();
        spawn_async!({
            let result = fetch(FetchRequest::get(src.to_string())).await;
            if result.is_ok() {
                match result.get_result() {
                    Some(svg) => {
                        if svg.starts_with(r#"<?xml version="1.0" encoding="UTF-8"?>"#) {
                            image.set(clean_html(&svg));
                        } else {
                            image.set(String::from(
                                r#"<i class="fa-regular fa-triangle-exlamation" title="Image data was not expected SVG content" />"#,
                            ))
                        }
                    },
                    None => image.set(String::from(
                        r#"<i class="fa-regular fa-triangle-exlamation" title="failed to load image" />"#,
                    )),
                }
            } else {
                image.set(String::from(
                    r#"<i class="fa-regular fa-triangle-exlamation" title="Failed to load image" />"#,
                ))
            }
        });
        return html! {
            <Loading variant={LoadingVariant::Circle} size={0} />
        };
    }
    html! {<HtmlContent html={image.to_string()} />}
}
