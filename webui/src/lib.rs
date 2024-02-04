//! # Web UI
//!
//! `webui` is a website framework for building webassembly SPA websites quickly and easily.
//! Development is just getting started, so we do not recommend using at this point for anything more than experimenting.
#![allow(unused)] // TODO: Remove me when needing to check for dead code / unused methods/variables.
use std::cell::Cell;

/// Actors represent intermediaries for processing specific types of requests
pub mod actors;
/// Helpers for common settings and configurations
pub mod common;
/// Components used for managing and rendering html output
pub mod components;
/// Constants used throughout the application
pub mod constants;
/// Data Types
pub mod data_types;
/// Various helper methods
pub mod general;
/// Global methods and helpers
pub mod global;
/// Internal loaders
pub(crate) mod loaders;
/// MyFi API components and integrations
#[cfg(feature = "myfi")]
mod myfi;
/// Shortcut for all common components
pub mod prelude;
/// Starter pages for new projects
#[cfg(feature = "pages")]
pub mod starter_pages;
/// Modules that hold application states
pub mod states;
/// Stoic Dreams specific features
#[cfg(feature = "stoic")]
mod stoic;

#[macro_use]
extern crate webui_procs;
pub use webui_procs::*;

/// Generalized macros
#[macro_use]
pub mod macros;
/// Javascript interop and related macros
#[macro_use]
pub mod interop;

use components::layout::app::start_webui_app;
pub use prelude::*;

/// Initializer to run in app main() to start application/website
///
/// example
/// ```rust,ignore
/// use webui::prelude::*;
///
/// fn main() {
///     let app_config = AppConfig::builder(
///            "Web UI Demo & Documentation".to_owned(),
///            "Stoic Dreams".to_owned(),
///            "https://www.stoicdreams.com".to_owned(),
///             "StoicDreams.com".to_owned(),
///             )
///             .set_header_logo_src("Logo.svg".to_owned())
///             .set_nav_routing(nav_menu::get_nav_routing())
///             .set_drawer_toggle_header_left(nav_menu::nav_menu_info())
///             .set_drawer_toggle_header_middle(feedback_button_info())
///             .build();
///     webui::start_app(app_config);
/// }
/// ```
pub fn start_app(app_config: AppConfig) {
    unsafe {
        set_app_name(app_config.app_name.to_owned());
        set_company_name(app_config.company_name.to_owned());
        set_domain(app_config.domain.to_owned());
    }
    start_webui_app(app_config, vec![]);
}

pub const VERSION: &str = "0.7.23";
#[cfg(feature = "tauri")]
pub const IS_TAURI_APP: bool = true;

#[cfg(not(feature = "tauri"))]
pub const IS_TAURI_APP: bool = false;

thread_local!(static COMPANY_PLURAL: Cell<&'static str> = Cell::new("Company's"));
thread_local!(static COMPANY_SINGULAR: Cell<&'static str> = Cell::new("Company"));
thread_local!(static APP_NAME: Cell<&'static str> = Cell::new("Web UI App"));
thread_local!(static DOMAIN: Cell<&'static str> = Cell::new("Domain"));

#[no_mangle]
unsafe fn set_app_name(value: String) {
    APP_NAME.with(|a: &Cell<&'static str>| a.set(Box::leak(value.into_boxed_str())));
}
#[no_mangle]
unsafe fn set_domain(value: String) {
    DOMAIN.with(|a: &Cell<&'static str>| a.set(Box::leak(value.into_boxed_str())));
}
#[no_mangle]
unsafe fn set_company_name(value: String) {
    let plural = format!("{}{}", value, if value.ends_with('s') { "'" } else { "'s" });
    COMPANY_PLURAL.with(|a: &Cell<&'static str>| a.set(Box::leak(plural.into_boxed_str())));
    COMPANY_SINGULAR.with(|a: &Cell<&'static str>| a.set(Box::leak(value.into_boxed_str())));
}

pub fn get_app_name() -> &'static str {
    APP_NAME.with(|a| a.get())
}

pub fn get_company_singular() -> &'static str {
    COMPANY_SINGULAR.with(|a| a.get())
}

pub fn get_company_plural() -> &'static str {
    COMPANY_PLURAL.with(|a| a.get())
}

pub fn get_domain() -> &'static str {
    DOMAIN.with(|a| a.get())
}
