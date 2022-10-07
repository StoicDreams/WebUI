use yew::{function_component, html, use_state, Callback, Children, Classes, Properties};

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

/// Common container component
#[function_component(NavLink)]
pub fn paper(props: &NavLinkProps) -> Html {
    let classes = &mut Classes::new();
    classes.push("navlink");

    if !props.class.is_empty() {
        classes.push(&props.class);
    }

    let test = use_state(|| 0);
    let onclick = {
        let test = test.clone();
        Callback::from(move |_| test.set(*test + 1))
    };

    html! {
        <a href={props.to.to_string()} class={classes.clone()} onclick={onclick}>
            { for props.children.iter() }
        </a>
    }
}
