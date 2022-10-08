use webui::{
    data_types::drawer_toggle_info::DrawerToggleInfo, roles, Direction, NavDisplay, NavGroupInfo,
    NavLinkInfo, NavRoute, Paper,
};
use yew::prelude::*;

use crate::pages::{about::page_about, home::page_home};

pub fn nav_menu_info() -> DrawerToggleInfo {
    DrawerToggleInfo {
        display: || {
            html! {
                <i class="fa-solid fa-bars"></i>
            }
        },
        title: "Open Navigation Menu".to_owned(),
        class: "".to_owned(),
        drawer: Direction::Left,
        drawer_content: nav_menu_render,
    }
}

pub(crate) fn get_nav_routing() -> Vec<NavRoute> {
    let nav_routes: &mut Vec<NavRoute> = &mut Vec::new();
    nav_routes.push(NavRoute::NavLink(NavLinkInfo::new(
        "Home",
        "/",
        "fa-solid fa-house",
        roles::PUBLIC,
        page_home,
    )));
    nav_routes.push(NavRoute::NavLink(NavLinkInfo::new(
        "About",
        "/about",
        "fa-solid fa-circle-info",
        roles::PUBLIC,
        page_about,
    )));
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
