use webui::AppConfig;

fn main() {
    webui::start_app(setup_app_config());
}

fn setup_app_config() -> AppConfig {
    let app_config: AppConfig = AppConfig {
        app_name: "Web UI Demo & Documentation".to_owned(),
        company_name: "Stoic Dreams".to_owned(),
        company_home_url: "https://www.stoicdreams.com".to_owned(),
        domain: "StoicDreams.com".to_owned(),
        hide_powered_by: false,
    };
    return app_config;
}