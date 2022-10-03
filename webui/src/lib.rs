//! # Web UI
//!
//! `webui` is a website framework for building webassembly SPA websites quickly and easily.
//! Development is just getting started, so we do not recommend using at this point for anything more than experimenting.

use crate::data_types::app_config::AppConfig;
use components::layout::app::{App, AppProps};

mod components;
pub mod data_types;

/// Initializer to run in app main() to start website
pub fn start_app(app_config: AppConfig) {
    let props = AppProps { config: app_config };
    yew::start_app_with_props::<App>(props);
}
