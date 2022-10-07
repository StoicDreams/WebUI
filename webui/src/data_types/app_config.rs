use crate::data_types::drawer_toggle_info::DrawerToggleInfo;

use super::nav_route::NavRoute;

/// Struct holding App/Website configuration details.
#[derive(Clone, Debug, PartialEq)]
pub struct AppConfig {
    pub app_name: String,
    pub company_name: String,
    pub company_home_url: String,
    pub domain: String,
    pub header_logo_src: Option<String>,
    pub hide_powered_by: bool,
    pub nav_routing: Vec<NavRoute>,
    pub header_left_drawer_toggle: Option<DrawerToggleInfo>,
}
