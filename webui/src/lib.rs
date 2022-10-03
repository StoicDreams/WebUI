//! # Web UI
//!
//! `webui` is a website framework for building webassembly SPA websites quickly and easily.
//! Development is just getting started, so we do not recommend using at this point for anything more than experimenting.

use components::layout::app::App;

mod components;

pub trait AppData {
    fn Title(&self) -> str;
    fn Body(&self) -> str;
}

/// Initializer to run in app main() to start website
pub fn start_app() {
    yew::start_app::<App>();
}
