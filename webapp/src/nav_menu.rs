use webui::{data_types::drawer_toggle_info::DrawerToggleInfo, NavLink};
use yew::prelude::*;

pub fn nav_menu_info() -> DrawerToggleInfo {
    DrawerToggleInfo {
        display: || {
            html! {
                <i class="fa-solid fa-bars" />
            }
        },
        class: "".to_string(),
        drawer_content: nav_menu_render,
    }
}

fn nav_menu_render() -> Html {
    html! {
        <NavMenu />
    }
}

#[function_component(NavMenu)]
fn nav_menu() -> Html {
    html! {
        <>
            <NavLink to="/">{"Home"}</NavLink>
            <NavLink to="/about">{"about"}</NavLink>
        </>
    }
}
