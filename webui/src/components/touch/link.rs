use crate::prelude::*;

/// Properties for NavLink component
#[derive(Properties, PartialEq)]
pub struct NavLinkProps {
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
pub fn link(props: &NavLinkProps) -> Html {
    let contexts = use_context::<Contexts>().expect("Contexts not found");
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
    let target = if props.target.is_empty() {
        "_self".to_owned()
    } else {
        props.target.to_owned()
    };
    let mypath = props.href.to_string();
    let is_external_link = mypath.contains("://");
    let is_open_in_new_tab = target == "_blank"
        || target == "_new"
        || (!mypath.starts_with("http") && mypath.contains("://"));
    let onclick = {
        let contexts = contexts.clone();
        let mypath = mypath.clone();
        Callback::from(move |_| {
            if is_external_link {
                let window = web_sys::window().expect("no global `window` exists");
                if is_open_in_new_tab {
                    let _ = window.open_with_url(&mypath);
                    return;
                }
                window.location().set_href(&mypath).unwrap();
                return;
            }
            nav_to!(contexts, mypath);
        })
    };
    html! {
        <a href={mypath}
            title={title}
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
