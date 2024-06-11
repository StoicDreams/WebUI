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
    let text = props.text.to_owned();
    let text2 = props.second.to_owned();
    html! {<webui-stoic-dreams-logo {text} {text2}></webui-stoic-dreams-logo>}
}
