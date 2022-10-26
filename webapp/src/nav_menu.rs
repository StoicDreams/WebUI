use webui::{
    data_types::drawer_toggle_info::DrawerToggleInfo, html, roles, Direction, Html, NavDisplay,
    NavGroupInfo, NavLinkInfo, NavRoute, Paper,
};

use crate::pages::*;

pub fn nav_menu_info() -> DrawerToggleInfo {
    DrawerToggleInfo::new(
        "Navigation Menu".to_owned(),
        || html! {<i class="fa-solid fa-bars"></i>},
        nav_menu_render,
    )
    .set_button_class("btn toggle theme-inherit".to_string())
    .hide_header()
    .hide_footer()
    .set_drawer(Direction::Left)
    .build()
}

pub(crate) fn get_nav_routing() -> Vec<NavRoute> {
    let nav_routes = vec![
        NavLinkInfo::link(
            "Home",
            "/",
            "fa-duotone fa-house",
            roles::PUBLIC,
            page_home,
        ),
        NavGroupInfo::link(
            "Classes",
            "fa-duotone fa-file-code",
            roles::PUBLIC,
            vec![NavLinkInfo::link(
                "Themes",
                "/classes/themes",
                "fa-duotone fa-palette",
                roles::PUBLIC,
                page_classes_themes,
            )],
        ),
        NavGroupInfo::link(
            "Components",
            "fa-duotone fa-toolbox",
            roles::PUBLIC,
            vec![NavGroupInfo::link(
                "Containers",
                "fa-duotone fa-box-open-full",
                roles::PUBLIC,
                vec![NavLinkInfo::link(
                    "Paper",
                    "/components/containers/paper",
                    "fa-duotone fa-memo",
                    roles::PUBLIC,
                    page_components_containers_paper,
                )],
            )],
        ),
        NavGroupInfo::link(
            "Blogs",
            "fa-duotone fa-blog",
            roles::PUBLIC,
            vec![NavLinkInfo::link(
                "What is a web framework?",
                "/blogs/what-is-a-website-framework",
                "fa-duotone fa-block-question",
                roles::PUBLIC,
                page_blogs_what_is_ui_framework,
            )],
        ),
        NavLinkInfo::link(
            "About",
            "/about",
            "fa-duotone fa-circle-info",
            roles::PUBLIC,
            page_about,
        ),
        NavLinkInfo::link(
            "Terms",
            "/terms",
            "fa-duotone fa-handshake",
            roles::PUBLIC,
            page_terms,
        ),
        NavLinkInfo::link(
            "Privacy",
            "/privacy",
            "fa-duotone fa-shield-exclamation",
            roles::PUBLIC,
            page_privacy,
        )
    ];
    nav_routes.to_owned()
}

fn nav_menu_render() -> Html {
    html! {
        <>
            <Paper class="d-flex pa-1 justify-center">
                <img src="Logo.svg" title="Web UI Logo" />
            </Paper>
            <NavDisplay routes={get_nav_routing()} class="d-flex flex-column pa-1" />
        </>
    }
}
