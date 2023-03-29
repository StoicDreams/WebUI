//! # Web UI
//!
//! `webui` is a website framework for building webassembly SPA websites quickly and easily.
//! Development is just getting started, so we do not recommend using at this point for anything more than experimenting.
use std::cell::Cell;

/// Actors represent intermediaries for processing specific types of requests
pub mod actors;
/// Helpers for common settings and configurations
pub mod common;
/// Components used for managing and rendering html output
pub mod components;
/// Data Types
pub mod data_types;
/// Global methods and helpers
pub mod global;
/// Shortcut for all common components
pub mod prelude;
/// Modules that hold application states
pub mod states;

/// Generalized macros
#[macro_use]
pub mod macros;
/// Javascript interop and related macros
#[macro_use]
pub mod interop;

pub use crate::states::*;
pub use async_std;
pub use chrono;
use components::layout::app::start_webui_app;
pub use futures;
pub use global::*;
pub use js_sys;
pub use js_sys::Function;
pub use lazy_static;
pub use lazy_static::*;
pub use num_format::*;
pub use prelude::*;
pub use regex;
pub use rust_decimal;
pub use rust_decimal_macros::*;
pub use serde;
pub use serde_json;
pub use wasm_bindgen;
pub use wasm_bindgen_futures;
pub use wasm_bindgen_futures::{spawn_local, JsFuture};
pub use wasm_logger;
pub use web_sys;
pub use web_sys::{Request, RequestInit, RequestMode, Response};
pub use yew;
pub use yew::macros::*;
pub use yew::*;
pub use yew_agent;
pub use yew_agent::*;
pub use yew_hooks;

/// Initializer to run in app main() to start website
///
/// example
/// ```rust,ignore
/// use webui::*;
///
/// fn main() {
///     let app_config = AppConfig::new(
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
    }
    start_webui_app(app_config);
}

thread_local!(static COMPANY_PLURAL: Cell<&'static str> = Cell::new("Company's"));
thread_local!(static COMPANY_SINGULAR: Cell<&'static str> = Cell::new("Company"));
thread_local!(static APP_NAME: Cell<&'static str> = Cell::new("Web UI App"));

#[no_mangle]
unsafe fn set_app_name(value: String) {
    APP_NAME.with(|a: &Cell<&'static str>| a.set(Box::leak(value.into_boxed_str())));
}
#[no_mangle]
unsafe fn set_company_name(value: String) {
    let plural = format!(
        "{}{}",
        value,
        if value.chars().last().unwrap() == 's' {
            "'"
        } else {
            "'s"
        }
    );
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
