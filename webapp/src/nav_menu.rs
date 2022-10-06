use crate::page_routes::Route;
use webui::data_types::drawer_toggle_info::DrawerToggleInfo;
use yew::{html, Html};
use yew_router::prelude::*;

pub fn nav_menu_info() -> DrawerToggleInfo {
    DrawerToggleInfo {
        display: || {
            html! {
                <i class="fa-solid fa-bars" />
            }
        },
        class: "".to_string(),
        drawer_content: nav_menu,
    }
}

pub(crate) fn nav_menu() -> Html {
    html! {
        <>
            <Link<Route> to={Route::Home}>{"Home"}</Link<Route>>
            <Link<Route> to={Route::About}>{"About"}</Link<Route>>
        </>
    }
}
