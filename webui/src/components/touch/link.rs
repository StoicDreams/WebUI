use crate::prelude::*;

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
    let onclick = {
        let mypath = props.href.to_string();
        let navigation = contexts.nav.clone();
        Callback::from(move |_| {
            let mymessage = NavigationMessage::PathUpdate(mypath.clone());
            jslog!("mypath: {}", mypath);
            let mymessage = mymessage.clone();
            navigation.set(mymessage);
        })
    };
    html! {
        <a href={props.href.to_owned()}
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
