pub mod stoic_components;
pub mod stoic_pages;

pub use stoic_components::*;
pub use stoic_pages::*;

use crate::{trim_left_padding, HtmlContent};
use yew::{function_component, html, AttrValue, Html, Properties};

#[derive(PartialEq, Properties, Clone)]
pub struct AppLogoOptions {
    #[prop_or_default]
    pub text: AttrValue,
    #[prop_or_default]
    pub second: AttrValue,
}

#[function_component(AppLogo)]
pub fn app_logo(props: &AppLogoOptions) -> Html {
    let text = if props.second.is_empty() {
        format!(
            r#"<tspan class="single" x="50" y="50">{}</tspan>"#,
            props.text
        )
    } else {
        trim_left_padding(&format!(
            r#"<tspan class="line1" x="75" y="50">{}</tspan>
            <tspan class="line2" x="75" y="108">{}</tspan>"#,
            props.text, props.second
        ))
    };
    let title = format!("{} Logo", format!("{} {}", props.text, props.second).trim());
    html! {<HtmlContent html={trim_left_padding(&format!(r#"
        <svg class="logo" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" viewBox="0 0 150 150" version="1.1" title="{title}">
        <g class="background">
            <path style="fill-rule:nonzero;fill:rgb(0%,0%,0%);fill-opacity:1;stroke-width:1;stroke-linecap:butt;stroke-linejoin:miter;stroke-opacity:1;stroke-miterlimit:10;" d="M 2158.991216 2528.477009 L 416.929839 2528.477009 C 212.472576 2528.477009 46.822784 2362.788733 46.822784 2158.383846 L 46.822784 416.270769 C 46.822784 211.933098 212.472576 46.177605 416.929839 46.177605 L 2158.991216 46.177605 C 2363.381685 46.177605 2529.098271 211.933098 2529.098271 416.270769 L 2529.098271 2158.383846 C 2529.098271 2362.788733 2363.381685 2528.477009 2158.991216 2528.477009 Z M 2158.991216 2528.477009 " transform="matrix(0.0584818,0,0,0.0581146,0,0.00000113284)"/>
        </g>
        <g class="bottom-right">
            <path style="fill-rule:nonzero;fill-opacity:1;stroke-width:1;stroke-linecap:butt;stroke-linejoin:miter;stroke-opacity:1;stroke-miterlimit:10;" d="M 2047.711961 2355.529368 L 1657.967391 2355.529368 C 1481.430133 2355.529368 1338.223216 2212.29135 1338.223216 2035.714027 L 1338.223216 1645.993693 C 1338.223216 1469.41637 1481.430133 1326.178351 1657.967391 1326.178351 L 2047.711961 1326.178351 C 2224.316013 1326.178351 2367.52293 1469.41637 2367.52293 1645.993693 L 2367.52293 2035.714027 C 2367.389341 2212.425782 2224.316013 2355.529368 2047.711961 2355.529368 Z M 2047.711961 2355.529368 " transform="matrix(0.0584818,0,0,0.0581146,0,0.00000113284)"/>
        </g>
        <g class="bottom-left">
            <path style="fill-rule:nonzero;fill-opacity:1;stroke-width:1;stroke-linecap:butt;stroke-linejoin:miter;stroke-opacity:1;stroke-miterlimit:10;" d="M 939.194242 2355.529368 L 549.516466 2355.529368 C 372.912414 2355.529368 229.705497 2212.29135 229.705497 2035.714027 L 229.705497 1645.993693 C 229.705497 1469.41637 372.912414 1326.178351 549.516466 1326.178351 L 939.194242 1326.178351 C 1115.798294 1326.178351 1259.005211 1469.41637 1259.005211 1645.993693 L 1259.005211 2035.714027 C 1259.005211 2212.425782 1115.798294 2355.529368 939.194242 2355.529368 Z M 939.194242 2355.529368 " transform="matrix(0.0584818,0,0,0.0581146,0,0.00000113284)"/>
        </g>
        <g class="top-right">
            <path style="fill-rule:nonzero;fill-opacity:1;stroke-width:1;stroke-linecap:butt;stroke-linejoin:miter;stroke-opacity:1;stroke-miterlimit:10;" d="M 2027.406503 1232.882072 L 1637.728727 1232.882072 C 1461.124674 1232.882072 1317.917758 1089.71127 1317.917758 913.066731 L 1317.917758 523.480829 C 1317.917758 346.903507 1461.124674 203.732704 1637.728727 203.732704 L 2027.406503 203.732704 C 2204.010555 203.732704 2347.217471 346.903507 2347.217471 523.480829 L 2347.217471 913.201164 C 2347.083883 1089.778486 2204.010555 1232.882072 2027.406503 1232.882072 Z M 2027.406503 1232.882072 " transform="matrix(0.0584818,0,0,0.0581146,0,0.00000113284)"/>
        </g>
        <g class="top-left">
            <path style="fill-rule:nonzero;fill-opacity:1;stroke-width:1;stroke-linecap:butt;stroke-linejoin:miter;stroke-opacity:1;stroke-miterlimit:10;" d="M 939.194242 1258.491498 L 549.516466 1258.491498 C 372.912414 1258.491498 229.705497 1115.320695 229.705497 938.676156 L 229.705497 549.090255 C 229.705497 372.512932 372.912414 229.274913 549.516466 229.274913 L 939.194242 229.274913 C 1115.798294 229.274913 1259.005211 372.512932 1259.005211 549.090255 L 1259.005211 938.810589 C 1259.005211 1115.387912 1115.798294 1258.491498 939.194242 1258.491498 Z M 939.194242 1258.491498 " transform="matrix(0.0584818,0,0,0.0581146,0,0.00000113284)"/>
        </g>
        <g class="diamond">
            <path style=" stroke:none;fill-rule:nonzero;fill-opacity:1;" d="M 58.703125 141.28125 L 7.859375 90.757812 C -1.179688 81.773438 -1.179688 67.199219 7.859375 58.214844 L 58.703125 7.6875 C 67.746094 -1.296875 82.414062 -1.296875 91.453125 7.6875 L 142.296875 58.214844 C 151.339844 67.199219 151.339844 81.773438 142.296875 90.757812 L 91.453125 141.28125 C 82.414062 150.265625 67.75 150.265625 58.703125 141.28125 Z M 58.703125 141.28125 "/>
        </g>
        <g class="text">
            <text x="50%" y="50%" fill="white" font-size="70" font-weight="700" text-anchor="middle" dominant-baseline="central">
                {text}
            </text>
        </g>
        </svg>
        "#))} />}
}
