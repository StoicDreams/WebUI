use web_sys::HtmlLinkElement;

use crate::prelude::*;

/// Properties for NavLink component
#[derive(Properties, PartialEq)]
pub struct LinkProps {
    pub href: String,
    #[prop_or_default]
    pub target: String,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: String,
    #[prop_or_default]
    pub title: String,
    #[prop_or_default]
    pub icon: String,
    #[prop_or_default]
    pub onclick: Option<fn(ev: MouseEvent)>,
}

#[function_component(Link)]
pub fn link(props: &LinkProps) -> Html {
    let contexts = use_context::<Contexts>().expect("Contexts not found");
    let app_config = contexts.clone().config;
    let classes = &mut Classes::new();
    classes.push("navlink");
    if !props.class.is_empty() {
        classes.push(&props.class);
    }
    let title = if props.title.is_empty() {
        props.href.to_owned()
    } else {
        props.title.to_owned()
    };
    let target = if app_config.external_links_new_tab_only {
        "_blank".to_owned()
    } else if props.target.is_empty() {
        "_self".to_owned()
    } else {
        props.target.to_owned()
    };
    let mypath = props.href.to_string();
    let is_external_link = mypath.contains("://");
    let onclick = {
        let contexts = contexts.clone();
        let mypath = mypath.clone();
        let target = target.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            if is_external_link {
                jslog!("Open external link {}.", mypath);
                open_external_link(&mypath, &target);
                return;
            }
            jslog!("Nav to {}", mypath);
            nav_to!(contexts, mypath);
        })
    };
    html! {
        <a href={mypath}
            title={title}
            target={target}
            class={props.class.to_owned()}
            onclick={onclick}>
            {if !props.icon.is_empty() {
                html! {<i class={props.icon.to_string()} />}
            } else {
                html! {}
            }}
            {for props.children.iter()}
        </a>
    }
}
