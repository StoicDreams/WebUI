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
/// Generalized macros
#[macro_use]
pub mod macros;
/// Javascript interop and related macros
#[macro_use]
pub mod interop;

pub use crate::actors::fetch::*;
pub use crate::actors::global_data::*;
pub use crate::agents::app_drawer_agent::*;
pub use crate::agents::app_state_agent::*;
pub use crate::common::classes::*;
pub use crate::common::elevation::*;
pub use crate::components::container::card::Card;
pub use crate::components::container::list::List;
pub use crate::components::container::paper::Paper;
pub use crate::components::container::quote::Quote;
pub use crate::components::display::avatar::Avatar;
pub use crate::components::display::image::Image;
pub use crate::components::display::nav_display::NavDisplay;
pub use crate::components::touch::app_drawer_button::AppDrawerButton;
pub use crate::components::touch::button::Button;
pub use crate::components::touch::input_field::InputField;
pub use crate::components::touch::input_message::InputMessage;
pub use crate::components::touch::input_text::InputText;
pub use crate::components::touch::link::Link;
pub use crate::components::touch::navlink::NavLink;
pub use crate::data_types::app_config::AppConfig;
pub use crate::data_types::direction::*;
pub use crate::data_types::drawer_toggle_info::DrawerToggleInfo;
pub use crate::data_types::errors::*;
pub use crate::data_types::nav_route::*;
pub use crate::data_types::roles;
pub use crate::data_types::theme::Theme;
pub use crate::interop::*;
pub use crate::macros::titles::*;
pub use crate::macros::*;
pub use async_std;
pub use async_std::prelude::*;
pub use futures;
pub use serde;
pub use serde_json;
pub use wasm_bindgen;
pub use wasm_bindgen::{prelude::*, JsCast};
pub use wasm_bindgen_futures::{spawn_local, JsFuture};
pub use web_sys;
pub use web_sys::{Request, RequestInit, RequestMode, Response};
pub use yew;
pub use yew::prelude::*;
pub use yew_agent;
pub use yew_agent::*;
pub use yew_hooks;
pub use yew_hooks::prelude::*;

use components::layout::app::start_webui_app;

pub fn empty_html() -> Html {
    html! {}
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
