use webui::AppConfig;

mod nav_menu;
pub(crate) mod pages;

fn main() {
    webui::start_app(setup_app_config());
}

fn setup_app_config() -> AppConfig {
    let app_config: AppConfig = AppConfig {
        app_name: "Web UI Demo & Documentation".to_owned(),
        company_name: "Stoic Dreams".to_owned(),
        company_home_url: "https://www.stoicdreams.com".to_owned(),
        domain: "StoicDreams.com".to_owned(),
        header_logo_src: Some("Logo.svg".to_owned()),
        hide_powered_by: false,
        nav_routing: nav_menu::get_nav_routing(),
        header_left_drawer_toggle: Some(nav_menu::nav_menu_info()),
    };
    app_config
}
