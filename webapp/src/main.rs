#![allow(unused)] // TODO: Remove me when needing to check for dead code / unused methods/variables.
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
    .set_nav_routing(NavRoutingCallback::new(nav_menu::get_nav_routing))
    .set_navigation(nav_menu::nav_content)
    .set_header(myfi_app_header)
    .set_copyright_start(2022)
    .register_component("Test", render_test)
    .build()
}
