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
    pub icon: String,
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
    let mypath = props.href.to_string();
    let onclick = move |ev: MouseEvent| {
        match clickoption {
            Some(method) => method(ev),
            None => (),
        };
        if mypath.starts_with("/") {
            let mut app_state_agent = AppStateAgent::dispatcher();
            app_state_agent.send(AppStateRequest::PathUpdate(mypath.to_string()))
        }
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
