use crate::{function_component, html, Classes, Html, NavLink, NavRoute, Paper, Properties};

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

    if !props.class.is_empty() {
        classes.push(&props.class);
    }

    html! {
        <section class={classes.clone()}>
            {nav_display_group(&props.routes)}
            <div class="grow" />
        </section>
    }
}

fn nav_display_group(routes: &Vec<NavRoute>) -> Html {
    html! {
        <>
            {
                routes.into_iter().map(|route| {
                    html!{display_nav_route(route)}
                }).collect::<Html>()
            }
        </>
    }
}

fn display_nav_route(route: &NavRoute) -> Html {
    match route {
        NavRoute::NavGroup(group_info) => {
            return html! {
                <Paper class="nav-group">
                    {nav_display_group(&group_info.children)}
                </Paper>
            };
        }
        NavRoute::NavLink(link_info) => {
            return html! {
                <Paper class="nav-link">
                    <NavLink to={link_info.path.to_string()}>
                        <i class={&link_info.icon} />
                        <span>{link_info.name.to_string()}</span>
                    </NavLink>
                </Paper>
            };
        }
    }
}
