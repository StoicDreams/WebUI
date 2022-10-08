use crate::*;

/// Properties for NavLink component
#[derive(Properties, PartialEq)]
pub struct NavLinkProps {
    pub href: String,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: String,
    #[prop_or_default]
    pub title: String,
    #[prop_or_default]
    pub onclick: Option<fn(ev: MouseEvent)>,
}

#[function_component(Link)]
pub fn link(props: &NavLinkProps) -> Html {
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
    let clickoption = props.onclick.to_owned();
    let onclick = move |ev: MouseEvent| match clickoption {
        Some(method) => method(ev),
        None => (),
    };
    html! {
        <a href={props.href.to_owned()}
            title={title}
            class={props.class.to_owned()}
            onclick={onclick}>
            {for props.children.iter()}
        </a>
    }
}
