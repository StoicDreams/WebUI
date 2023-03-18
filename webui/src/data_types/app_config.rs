use crate::data_types::drawer_toggle_info::DrawerToggleInfo;
use crate::*;

/// Struct holding App/Website configuration details.
///
/// This is required to be created on app startup and passed into webui::start_app(app_config)
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
    pub header_right_drawer_toggle: Option<DrawerToggleInfo>,
    pub header_top_drawer_toggle: Option<DrawerToggleInfo>,
    pub footer_left_drawer_toggle: Option<DrawerToggleInfo>,
    pub footer_right_drawer_toggle: Option<DrawerToggleInfo>,
    pub footer_bottom_drawer_toggle: Option<DrawerToggleInfo>,
    pub header_strip_bar: Option<fn() -> Html>,
    pub user_info_panel: Option<fn() -> Html>,
    pub copyright_year_start: Option<i16>,
}

/// Struct holding App/Website configuration details.
///
/// This is a builder object that allows configurating optional data through methods.
/// Run the .build() command when finished to return an AppConfig instance.
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
    pub(crate) header_strip_bar: Option<fn() -> Html>,
    pub(crate) user_info_panel: Option<fn() -> Html>,
    pub(crate) copyright_year_start: Option<i16>,
}
impl AppConfig {
    /// Create an AppConfigBuilder instance to build your AppConfig with.
    ///
    /// ```rust
    /// use webui::*;
    ///
    /// let app_config:AppConfig = AppConfig::new(
    ///     "App Name".to_string(),
    ///     "Company Name".to_string(),
    ///     "https://company.url".to_string(),
    ///     "company.url".to_string(),
    /// )
    /// .set_header_logo_src("Logo.svg".to_owned()).to_owned()
    /// .build();
    /// ```
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
            header_strip_bar: None,
            user_info_panel: None,
            copyright_year_start: None,
        }
    }
    pub fn get_nav_from_path(&self, path: &str) -> Option<NavLinkInfo> {
        get_nav_from_list(path, &self.nav_routing)
    }
}

fn get_nav_from_list(path: &str, list: &Vec<NavRoute>) -> Option<NavLinkInfo> {
    for nav in list.iter() {
        match nav {
            NavRoute::NavGroup(group) => match get_nav_from_list(path, &group.children) {
                Some(link) => {
                    return Some(link.to_owned());
                }
                None => (),
            },
            NavRoute::NavLink(link) => {
                if path.to_lowercase() == link.path.to_lowercase() {
                    return Some(link.to_owned());
                }
            }
        }
    }
    None
}

impl AppConfigBuilder {
    /// Build into an AppConfig instance
    ///
    /// Run this to finalize your app configuration.
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
            header_strip_bar: self.header_strip_bar.to_owned(),
            user_info_panel: self.user_info_panel.to_owned(),
            copyright_year_start: self.copyright_year_start.to_owned(),
        }
    }

    /// Set a URL to use for your website logo
    ///
    /// If set, this is displayed in the header on the far left side before any other content.
    pub fn set_header_logo_src(self: &mut Self, img_src: String) -> &mut Self {
        self.header_logo_src = Some(img_src);
        self
    }
    /// Set settings for navigation routing
    pub fn set_nav_routing(self: &mut Self, nav_routing: Vec<NavRoute>) -> &mut Self {
        self.nav_routing = nav_routing;
        self
    }
    /// Hide the Powered By Web UI link in the footer
    ///
    /// We appreciate you not using this to help spread the word about this framework, but we won't hold it against you if you do.
    pub fn hide_powered_by(self: &mut Self) -> &mut Self {
        self.hide_powered_by = true;
        self
    }
    /// Set a handler for a drawer toggle button
    ///
    /// This button will be displayed in the header, as the first item.
    pub fn set_drawer_toggle_header_left(
        self: &mut Self,
        drawer_info: DrawerToggleInfo,
    ) -> &mut Self {
        self.header_left_drawer_toggle = Some(drawer_info);
        self
    }
    /// Set a handler for a drawer toggle button
    ///
    /// This button will be displayed in the header, as the first item in the right side content grouping.
    /// Note: This button can be semi-centered on wider displays by setting builder.set_header_strip_bar(fn->Html) to include a .flex-grow classed element as the first item.
    pub fn set_drawer_toggle_header_middle(
        self: &mut Self,
        drawer_info: DrawerToggleInfo,
    ) -> &mut Self {
        self.header_top_drawer_toggle = Some(drawer_info);
        self
    }
    /// Set a handler for a drawer toggle button
    ///
    /// This button will be displayed in the header, as the last item.
    pub fn set_drawer_toggle_header_right(
        self: &mut Self,
        drawer_info: DrawerToggleInfo,
    ) -> &mut Self {
        self.header_right_drawer_toggle = Some(drawer_info);
        self
    }
    /// Set a handler for a drawer toggle button
    ///
    /// This button will be displayed in the footer, as the first item.
    pub fn set_drawer_toggle_footer_left(
        self: &mut Self,
        drawer_info: DrawerToggleInfo,
    ) -> &mut Self {
        self.footer_left_drawer_toggle = Some(drawer_info);
        self
    }
    /// Set a handler for a drawer toggle button
    ///
    /// This button will be displayed in the footer, immediately following the company name display.
    pub fn set_drawer_toggle_footer_middle(
        self: &mut Self,
        drawer_info: DrawerToggleInfo,
    ) -> &mut Self {
        self.footer_bottom_drawer_toggle = Some(drawer_info);
        self
    }
    /// Set a handler for a drawer toggle button
    ///
    /// This button will be displayed in the footer, as the last item.
    pub fn set_drawer_toggle_footer_right(
        self: &mut Self,
        drawer_info: DrawerToggleInfo,
    ) -> &mut Self {
        self.footer_right_drawer_toggle = Some(drawer_info);
        self
    }
    /// Set extra content to display in the header, between the middle togle button and the user info panel
    pub fn set_header_strip_bar(self: &mut Self, strip_bar: fn() -> Html) -> &mut Self {
        self.header_strip_bar = Some(strip_bar);
        self
    }
    /// Set extra content to display in the header, between the middle togle button and the user info panel
    pub fn set_user_info_panel(self: &mut Self, info_panel: fn() -> Html) -> &mut Self {
        self.user_info_panel = Some(info_panel);
        self
    }
    /// Set copyright years
    pub fn set_copyright_start(self: &mut Self, copyright_start: i16) -> &mut Self {
        self.copyright_year_start = Some(copyright_start);
        self
    }
}
