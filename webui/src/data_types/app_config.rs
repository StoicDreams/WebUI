use yew::Html;

/// Struct holding App/Website configuration details.
#[derive(Clone, Debug, PartialEq)]
pub struct AppConfig {
    pub app_name: String,
    pub company_name: String,
    pub company_home_url: String,
    pub domain: String,
    pub hide_powered_by: bool,
    pub body_html: fn() -> Html,
}
