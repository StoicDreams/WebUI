use crate::prelude::*;
use webui::prelude::*;

mod components;
mod nav_menu;
mod pages;
mod prelude;

fn main() {
    webui::start_app(setup_app_config());
}

fn setup_app_config() -> AppConfig {
    AppConfig::builder(
        "Web UI Demo & Documentation".to_owned(),
        "Stoic Dreams".to_owned(),
        "https://www.stoicdreams.com".to_owned(),
        "WebUI.StoicDreams.com".to_owned(),
    )
    .set_header_logo_src("Logo.svg".to_owned())
    .set_nav_routing(nav_menu::get_nav_routing())
    .set_drawer_toggle_header_left(nav_menu::nav_menu_info())
    .set_drawer_toggle_header_middle(myfi_feedback_button_info())
    .set_header_strip_bar(stoic_header_strip_bar)
    .set_user_info_panel(myfi_info_panel)
    .set_copyright_start(2022)
    .register_component("Test", render_test)
    .build()
}
