use crate::{
    data_types::{
        app_config,
        nav_route::{NavLinkInfo, NavRoute},
    },
    AppConfig,
};
use yew::{function_component, html, use_context, Html};

/// App page body component - page specific content is rendered here
#[function_component(AppBody)]
pub(crate) fn app_body() -> Html {
    let app_config = use_context::<AppConfig>().expect("no app config found");

    html! {
        <main>
            {(get_page_content(app_config.nav_routing, "/"))()}
        </main>
    }
}

fn page_not_found() -> Html {
    html! {
        {"Page Not Found"}
    }
}

fn get_page_content(routes: Vec<NavRoute>, page: &str) -> fn() -> Html {
    let page = get_page(routes, page);
    match page {
        Option::Some(link_info) => link_info.page,
        Option::None => page_not_found,
    }
}

fn get_page(routes: Vec<NavRoute>, page: &str) -> Option<NavLinkInfo> {
    for route in routes {
        match route {
            NavRoute::NavLink(link_info) => {}
            NavRoute::NavGroup(group_info) => {
                if group_info.children.len() == 0 {
                    continue;
                }
                if let Option::Some(link_info) = get_page(group_info.children, page) {
                    return Option::Some(link_info);
                }
            }
        }
    }
    Option::None
}
