use crate::*;

/// Properties for Loading components
#[derive(Properties, PartialEq)]
pub struct LoadingProps {
    #[prop_or_default]
    pub variant: LoadingVariant,
    #[prop_or(16u16)]
    pub size: u16,
    #[prop_or(4u8)]
    pub stroke: u8,
    #[prop_or_default]
    pub color: Theme,
    #[prop_or_default]
    pub class: String,
    #[prop_or_default]
    pub style: String,
    #[prop_or_default]
    pub percent: Option<u8>,
    #[prop_or_default]
    pub offset: Option<u8>,
}

#[derive(Default, Clone, Debug, PartialEq)]
pub enum LoadingVariant {
    #[default]
    Circle,
    Bar,
    StripedBar,
}

/// u16 value 4
pub const LOADING_SIZE_TINY: u16 = 8;
/// u16 value 8
pub const LOADING_SIZE_SMALL: u16 = 16;
/// u16 value 16
pub const LOADING_SIZE_MEDIUM: u16 = 32;
/// u16 value 32
pub const LOADING_SIZE_LARGE: u16 = 64;
/// u16 value 64
pub const LOADING_SIZE_XLARGE: u16 = 128;

/// Display loading graphic
#[function_component(Loading)]
pub fn loading(props: &LoadingProps) -> Html {
    let size = props.size;
    let color = props.color.to_string().replace("theme-", "");
    let parent_class = classes!(
        "loading d-flex d-flex align-center justify-center",
        props.class
    );
    let parent_styles = props.style.to_owned();
    let mut class = match props.variant {
        LoadingVariant::Bar => classes!("loading bar", format!("theme-{}", color)),
        LoadingVariant::StripedBar => {
            classes!("loading bar striped", format!("theme-{}", color))
        }
        LoadingVariant::Circle => classes!("loading circle", format!("color-{}", color)),
    };
    let styles = match props.variant {
        LoadingVariant::Bar => String::from(format!("height:{size}px")),
        LoadingVariant::StripedBar => String::from(format!("height:{size}px")),
        LoadingVariant::Circle => String::from(format!(
            "width:{size}px;height:{size}px;max-width:{size}px;max-height:{size}px;"
        )),
    };
    let mut style_offset = String::default();
    let mut style_percent = String::default();
    match props.percent {
        Some(percent) => match props.variant {
            LoadingVariant::Circle => {
                let percent = f32::from(percent) / 100f32 * 138f32;
                style_percent = format!("stroke-dasharray: {percent}px,138px;");
            }
            _ => {
                style_percent = format!("width:{percent}%;");
                match props.offset {
                    Some(offset) => {
                        style_offset = format!("width:{offset}%;");
                    }
                    None => (),
                }
            }
        },
        None => {
            class.push("indeterminate");
        }
    }

    html!(
        <Paper class={parent_class.to_string()} style={parent_styles}>
            <Paper class={class.to_string()} style={styles}>
                {match props.variant {
                    LoadingVariant::Bar | LoadingVariant::StripedBar => html!(
                        <>
                            <div style={style_offset} />
                            <div style={style_percent} />
                            <div/>
                        </>
                    ),
                    LoadingVariant::Circle => {
                        let width = "44";
                        let radius = "22";
                        let stroke = props.stroke.to_string();
                        html!(
                            <svg class="circle-svg" viewBox={format!("{radius} {radius} {width} {width}")}>
                                <circle style={style_percent} cx={width.to_owned()} cy={width} r={radius} fill="none" stroke-width={stroke} stoke-linecap="round"></circle>
                            </svg>
                        )
                    },
                }}
            </Paper>
        </Paper>
    )
}
