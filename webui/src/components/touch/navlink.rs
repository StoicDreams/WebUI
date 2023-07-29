use crate::prelude::*;

/// Properties for NavLink component
#[derive(Properties, PartialEq)]
pub struct NavLinkProps {
    #[prop_or_default]
    pub to: String,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: String,
}

/// Navigation link component
///
/// Use this when you want to display a navigation button or link
/// Make sure the `to` property starts with a forward slash `/`
///
/// example to display a link
/// ```rust
/// use webui::prelude::*;
///
/// fn component() -> Html {
///     html! {
///         <NavLink to="/some-page">{"A Link"}</NavLink>
///     }
/// }
/// ```
///
/// example to display a button
/// ```rust
/// use webui::prelude::*;
///
/// fn component() -> Html {
///     html! {
///         <NavLink to="/some-page">{"A Link"}</NavLink>
///     }
/// }
/// ```
#[function_component(NavLink)]
pub fn link(props: &NavLinkProps) -> Html {
    let contexts = use_context::<Contexts>().expect("Contexts not found");
    let is_active = use_state(|| false);
    let classes = &mut Classes::new();
    classes.push("navlink");

    if !props.class.is_empty() {
        classes.push(&props.class);
    }

    if is_active.deref().to_owned() {
        classes.push("active".to_owned());
    }

    let onclick = {
        let mypath = props.to.to_owned();
        let navigation = contexts.nav.clone();
        let mymessage = NavigationMessage::PathUpdate(mypath);
        Callback::from(move |_| {
            let mymessage = mymessage.clone();
            contexts.drawer.set(DrawerMessage::Close);
            navigation.set(mymessage);
        })
    };

    html! {
        <a href={props.to.to_string()} class={classes.clone()} onclick={onclick}>
            { for props.children.iter() }
        </a>
    }
}
