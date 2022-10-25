use crate::*;

/// Properties for SideImage components
#[derive(Properties, PartialEq)]
pub struct SideImageProps {
    /// Source of image to display
    pub src: String,
    #[prop_or_default]
    pub elevation: u8,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub content_class: Option<String>,
    #[prop_or_default]
    pub image_class: Option<String>,
    #[prop_or_default]
    pub title: String,
    #[prop_or_default]
    pub style: String,
    #[prop_or_default]
    pub image_pos: LeftOrRight,
}

/// Display SideImage
#[function_component(SideImage)]
pub fn side_image(props: &SideImageProps) -> Html {
    let classes = &mut Classes::new();
    let content_classes = &mut Classes::new();
    let image_classes = &mut Classes::new();
    classes.push(CLASSES_SIDE_BY_SIDE);
    match &props.class {
        Some(class) => {
            classes.push(class);
        }
        None => {}
    };
    match &props.content_class {
        Some(class) => {
            content_classes.push(class);
        }
        None => {
            content_classes.push("d-flex flex-column align-center justify-center readable-content");
        }
    };
    match &props.image_class {
        Some(class) => {
            image_classes.push(class);
        }
        None => {
            image_classes.push("d-flex flex-column align-center justify-center readable-content");
        }
    };

    if props.elevation > 0 {
        classes.push(format!("elevation-{}", props.elevation));
    }

    html! {
        <Paper class={classes.to_string()} elevation={ELEVATION_STANDARD}>
            {match &props.image_pos {
                LeftOrRight::Left => {
                    html! {
                        <>
                            <Image class={image_classes.to_string()} src={props.src.to_string()} />
                            <Paper class={content_classes.to_string()}>
                                {for props.children.iter()}
                            </Paper>
                        </>
                    }
                },
                LeftOrRight::Right => {
                    html! {
                        <>
                            <Paper class={content_classes.to_string()}>
                                {for props.children.iter()}
                            </Paper>
                            <Image class={image_classes.to_string()} src={props.src.to_string()} />
                        </>
                    }
                }
            }}
        </Paper>
    }
}
