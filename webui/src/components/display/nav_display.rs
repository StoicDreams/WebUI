use yew::Callback;

use crate::prelude::*;

/// Properties for NavDislay component
#[derive(Properties, PartialEq)]
pub struct NavDisplayProps {
    #[prop_or_default]
    pub routes: Vec<NavRoute>,
    #[prop_or_default]
    pub class: String,
}

/// Component for display a navigation tree
#[function_component(NavDisplay)]
pub fn nav_display(props: &NavDisplayProps) -> Html {
    let classes = &mut Classes::new();
    classes.push("nav-display");
    let contexts = use_context::<Contexts>();
    let user_roles = match contexts {
        Some(contexts) => contexts.user_roles.deref().to_owned(),
        None => 0,
    };
    if !props.class.is_empty() {
        classes.push(&props.class);
    }

    html! {
        <section class={classes.clone()}>
            {nav_display_group(&props.routes, &user_roles)}
            <div class="grow" />
        </section>
    }
}

fn nav_display_group(routes: &[NavRoute], user_roles: &i32) -> Html {
    html! {
        <>
            {
                routes.iter().map(|route| {
                    html!{display_nav_route(route, user_roles)}
                }).collect::<Html>()
            }
        </>
    }
}

fn display_nav_route(route: &NavRoute, user_roles: &i32) -> Html {
    match route {
        NavRoute::NavGroup(group_info) => {
            if group_info.role != 0 && group_info.role & user_roles == 0 {
                return html! {};
            }
            html! {
                <Paper class="nav-group">
                    <DisplayNavGroupToggle group_info={group_info.to_owned()} />
                    {nav_display_group(&group_info.children, user_roles)}
                </Paper>
            }
        }
        NavRoute::NavLink(link_info) => {
            if link_info.role != 0 && link_info.role & user_roles == 0 {
                return html! {};
            }
            html! {
                <Paper class="nav-link">
                    <NavLink to={link_info.path.to_string()}>
                        {link_info.icon.to_html()}
                        <span>{link_info.name.to_string()}</span>
                    </NavLink>
                </Paper>
            }
        }
    }
}

/// Properties for NavDislay component
#[derive(Properties, PartialEq)]
pub struct DisplayNavGroupToggleProps {
    pub group_info: NavGroupInfo,
}

#[function_component(DisplayNavGroupToggle)]
fn display_nav_group_toggle(props: &DisplayNavGroupToggleProps) -> Html {
    let show_children = use_state(|| false);
    let onclick = {
        let show_children = show_children.clone();
        Callback::from(move |_| show_children.set(!*show_children))
    };
    let classes = &mut Classes::new();
    classes.push("navlink");
    if *show_children {
        classes.push("show");
    }
    html! {
        <a {onclick} class={classes.to_string()}>
            {props.group_info.icon.to_html()}
            <span>{props.group_info.name.to_string()}</span>
            {display_caret(*show_children)}
        </a>
    }
}

fn display_caret(is_showing: bool) -> Html {
    match is_showing {
        true => html! {FaIcon::solid("caret-up").to_html()},
        false => html! {FaIcon::solid("caret-down").to_html()},
    }
}
