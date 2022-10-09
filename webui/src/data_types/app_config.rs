use crate::data_types::drawer_toggle_info::DrawerToggleInfo;

use super::nav_route::NavRoute;

/// Struct holding App/Website configuration details.
#[derive(Clone, Debug, PartialEq)]
pub struct AppConfig {
    pub(crate) app_name: String,
    pub(crate) company_name: String,
    pub(crate) company_home_url: String,
    pub(crate) domain: String,
    pub(crate) header_logo_src: Option<String>,
    pub(crate) hide_powered_by: bool,
    pub(crate) nav_routing: Vec<NavRoute>,
    pub(crate) header_left_drawer_toggle: Option<DrawerToggleInfo>,
    pub(crate) header_right_drawer_toggle: Option<DrawerToggleInfo>,
    pub(crate) header_top_drawer_toggle: Option<DrawerToggleInfo>,
    pub(crate) footer_left_drawer_toggle: Option<DrawerToggleInfo>,
    pub(crate) footer_right_drawer_toggle: Option<DrawerToggleInfo>,
    pub(crate) footer_bottom_drawer_toggle: Option<DrawerToggleInfo>,
}

/// Struct holding App/Website configuration details.
#[derive(Clone, Debug, PartialEq)]
pub struct AppConfigBuilder {
    pub(crate) app_name: String,
    pub(crate) company_name: String,
    pub(crate) company_home_url: String,
    pub(crate) domain: String,
    pub(crate) header_logo_src: Option<String>,
    pub(crate) hide_powered_by: bool,
    pub(crate) nav_routing: Vec<NavRoute>,
    pub(crate) header_left_drawer_toggle: Option<DrawerToggleInfo>,
    pub(crate) header_right_drawer_toggle: Option<DrawerToggleInfo>,
    pub(crate) header_top_drawer_toggle: Option<DrawerToggleInfo>,
    pub(crate) footer_left_drawer_toggle: Option<DrawerToggleInfo>,
    pub(crate) footer_right_drawer_toggle: Option<DrawerToggleInfo>,
    pub(crate) footer_bottom_drawer_toggle: Option<DrawerToggleInfo>,
}
impl AppConfig {
    pub fn new(
        app_name: String,
        company_name: String,
        company_home_url: String,
        domain: String,
    ) -> AppConfigBuilder {
        AppConfigBuilder {
            app_name,
            company_name,
            company_home_url,
            domain,
            header_logo_src: None,
            hide_powered_by: false,
            nav_routing: Vec::new(),
            header_left_drawer_toggle: None,
            header_right_drawer_toggle: None,
            header_top_drawer_toggle: None,
            footer_left_drawer_toggle: None,
            footer_right_drawer_toggle: None,
            footer_bottom_drawer_toggle: None,
        }
    }
}

impl AppConfigBuilder {
    pub fn build(self: &mut Self) -> AppConfig {
        AppConfig {
            app_name: self.app_name.to_string(),
            company_name: self.company_name.to_string(),
            company_home_url: self.company_home_url.to_string(),
            domain: self.domain.to_string(),
            header_logo_src: self.header_logo_src.to_owned(),
            hide_powered_by: self.hide_powered_by.to_owned(),
            nav_routing: self.nav_routing.to_owned(),
            header_left_drawer_toggle: self.header_left_drawer_toggle.to_owned(),
            header_right_drawer_toggle: self.header_right_drawer_toggle.to_owned(),
            header_top_drawer_toggle: self.header_top_drawer_toggle.to_owned(),
            footer_left_drawer_toggle: self.footer_left_drawer_toggle.to_owned(),
            footer_right_drawer_toggle: self.footer_right_drawer_toggle.to_owned(),
            footer_bottom_drawer_toggle: self.footer_bottom_drawer_toggle.to_owned(),
        }
    }
    pub fn set_header_logo_src(self: &mut Self, img_src: String) -> &mut Self {
        self.header_logo_src = Some(img_src);
        self
    }
    pub fn set_nav_routing(self: &mut Self, nav_routing: Vec<NavRoute>) -> &mut Self {
        self.nav_routing = nav_routing;
        self
    }
    pub fn hide_powered_by(self: &mut Self) -> &mut Self {
        self.hide_powered_by = true;
        self
    }
    pub fn set_drawer_toggle_header_left(
        self: &mut Self,
        drawer_info: DrawerToggleInfo,
    ) -> &mut Self {
        self.header_left_drawer_toggle = Some(drawer_info);
        self
    }
    pub fn set_drawer_toggle_header_middle(
        self: &mut Self,
        drawer_info: DrawerToggleInfo,
    ) -> &mut Self {
        self.header_top_drawer_toggle = Some(drawer_info);
        self
    }
}
