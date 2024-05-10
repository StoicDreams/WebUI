use std::fmt::Display;

use serde_json::to_vec;

use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct FaIcon {
    pub name: String,
    pub family: String,
    pub class: String,
}

impl FaIcon {
    pub fn none() -> Self {
        Self {
            name: String::default(),
            family: String::default(),
            class: String::default(),
        }
    }
    pub fn brands(name: &str) -> Self {
        Self {
            name: name.to_string(),
            family: String::from("brands"),
            class: String::default(),
        }
    }
    pub fn solid(name: &str) -> Self {
        Self {
            name: name.to_string(),
            family: String::from("solid"),
            class: String::default(),
        }
    }
    pub fn regular(name: &str) -> Self {
        Self {
            name: name.to_string(),
            family: String::from("regular"),
            class: String::default(),
        }
    }
    pub fn light(name: &str) -> Self {
        Self {
            name: name.to_string(),
            family: String::from("light"),
            class: String::default(),
        }
    }
    pub fn thin(name: &str) -> Self {
        Self {
            name: name.to_string(),
            family: String::from("thin"),
            class: String::default(),
        }
    }
    pub fn duotone(name: &str) -> Self {
        Self {
            name: name.to_string(),
            family: String::from("duotone"),
            class: String::default(),
        }
    }
    pub fn from(value: &str) -> Self {
        let mut segments = value.split(' ').collect::<Vec<_>>();
        let families = [
            "fa-duotone",
            "fa-brands",
            "fa-solid",
            "fa-regular",
            "fa-light",
            "fa-thin",
            "duotone",
            "brands",
            "solid",
            "regular",
            "light",
            "thin",
        ];
        let family = segments
            .iter()
            .find(|item| families.contains(item))
            .unwrap_or(&"regular")
            .to_string();
        let icon = segments
            .iter()
            .find(|item| item.starts_with("fa-") && !families.contains(item))
            .unwrap_or(
                segments
                    .iter()
                    .find(|item| !families.contains(item))
                    .unwrap_or(&""),
            )
            .to_string();
        let name = icon.replace("fa-", "");
        segments.retain(|item| *item != icon && *item != family);
        let family = family.replace("fa-", "");
        let class = segments.join(" ");
        Self {
            name,
            family,
            class,
        }
    }
    pub fn class(&mut self, class: &str) -> &mut Self {
        self.class = class.to_string();
        self
    }
    pub fn to_html(&self) -> Html {
        let icon = self.name.to_string();
        let family = self.family.to_string();
        let class = self.class.to_string();
        html!(<webui-fa {icon} {family} {class}></webui-fa>)
    }
}

impl Display for FaIcon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "fa-{} fa-{}", self.name, self.family)
    }
}

/// Component to display an icon
///
/// Expecting a Font Awesome or other CSS classes.
#[derive(Properties, PartialEq, Default)]
pub struct IconOptions {
    /// Font Awesome Icon
    pub icon: FaIcon,
    #[prop_or_default]
    pub elevation: u8,
    #[prop_or(Theme::None)]
    pub color: Theme,
    #[prop_or_default]
    pub title: AttrValue,
    #[prop_or_default]
    pub style: AttrValue,
    #[prop_or_default]
    pub class: AttrValue,
}

#[function_component(Icon)]
pub fn icon(props: &IconOptions) -> Html {
    let classes = &mut Classes::new();

    if !props.class.is_empty() {
        classes.push(props.class.to_string());
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
            {props.icon.to_html()}
        </div>
    }
}
