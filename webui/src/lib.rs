//! # Web UI
//!
//! `webui` is a website framework for building webassembly SPA websites quickly and easily.
//! Development is just getting started, so we do not recommend using at this point for anything more than experimenting.

/// Actors represent intermediaries for processing specific types of requests
pub mod actors;
/// Agents for sending and receiving messages between components.
pub mod agents;
/// Helpers for common settings and configurations
pub mod common;
/// Components used for managing and rendering html output
pub mod components;
/// Data Types
pub mod data_types;
/// Shortcut for all common components
pub mod prelude;

/// Generalized macros
#[macro_use]
pub mod macros;
/// Javascript interop and related macros
#[macro_use]
pub mod interop;

pub use async_std;
use components::layout::app::start_webui_app;
pub use futures;
use js_sys::Function;
pub use lazy_static::*;
pub use prelude::*;
pub use rust_decimal_macros::*;
pub use serde;
pub use serde_json;
pub use wasm_bindgen;
pub use wasm_bindgen_futures::{spawn_local, JsFuture};
pub use web_sys;
pub use web_sys::{Request, RequestInit, RequestMode, Response};
pub use yew;
pub use yew_agent;
pub use yew_agent::*;
pub use yew_hooks;

pub fn empty_html() -> Html {
    html! {}
}

pub fn get_window() -> web_sys::Window {
    web_sys::window().unwrap()
}

pub fn set_timeout(handler: &Function, milliseconds: i32) -> Result<i32, JsValue> {
    let window = get_window();
    window.set_timeout_with_callback_and_timeout_and_arguments_0(handler, milliseconds)
}

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
    start_webui_app(app_config);
}
