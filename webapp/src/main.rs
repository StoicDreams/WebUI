use feedback::feedback_button_info;
use webui::*;

mod components;
mod feedback;
mod nav_menu;
mod pages;

pub use crate::components::jasper_link::*;
pub use crate::components::next_page::*;
pub use crate::components::*;

pub(crate) const COMPANY_PLURAL: &str = "Stoic Dreams'";
pub(crate) const COMPANY_SINGULAR: &str = "Stoic Dreams";
pub(crate) const APP_NAME: &str = "Web UI";

fn main() {
    webui::start_app(setup_app_config());
}

fn setup_app_config() -> AppConfig {
    AppConfig::new(
        "Web UI Demo & Documentation".to_owned(),
        "Stoic Dreams".to_owned(),
        "https://www.stoicdreams.com".to_owned(),
        "StoicDreams.com".to_owned(),
    )
    .set_header_logo_src("Logo.svg".to_owned())
    .set_nav_routing(nav_menu::get_nav_routing())
    .set_drawer_toggle_header_left(nav_menu::nav_menu_info())
    .set_drawer_toggle_header_middle(feedback_button_info())
    .set_header_strip_bar(header_strip_bar::header_strip_bar)
    .set_user_info_panel(info_panel::info_panel)
    .build()
}
