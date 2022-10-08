//! # Web UI
//!
//! `webui` is a website framework for building webassembly SPA websites quickly and easily.
//! Development is just getting started, so we do not recommend using at this point for anything more than experimenting.

pub mod agents;
pub mod components;
pub mod interop;

pub(crate) use crate::agents::app_drawer_agent;
pub use crate::agents::app_state_agent;
pub use crate::components::container::paper::Paper;
pub use crate::components::display::nav_display::NavDisplay;
pub use crate::components::touch::navlink::NavLink;
pub use crate::data_types::app_config::AppConfig;
pub use crate::data_types::direction::Direction;
pub use crate::data_types::drawer_toggle_info::DrawerToggleInfo;
pub use crate::data_types::nav_route::*;
pub use crate::data_types::roles;

use components::layout::app::start_webui_app;

/// Data Types
pub mod data_types;

/// Initializer to run in app main() to start website
///
/// example
/// ```rust,ignore
/// use webui::AppConfig;
///
/// fn main() {
///     let app_config: AppConfig = AppConfig {
///         app_name: "Web UI".to_owned(),
///         company_name: "Sample Company".to_owned(),
///         company_home_url: "https://www.stoicdreams.com".to_owned(),
///         domain: "StoicDreams.com".to_owned(),
///         hide_powered_by: false,
///         body_html: page_routes::body_html,
///         header_left_drawer_toggle: Option::Some(nav_menu::nav_menu_info()),
///     };
///     webui::start_app(app_config);
/// }
/// ```
pub fn start_app(app_config: AppConfig) {
    start_webui_app(app_config);
}
