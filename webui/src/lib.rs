//! # Web UI
//!
//! `webui` is a website framework for building webassembly SPA websites quickly and easily.
//! Development is just getting started, so we do not recommend using at this point for anything more than experimenting.

use yew::prelude::*;

//include!(concat!(env!("OUT_DIR"), "/hello.rs"));

/// Initializer to run in app main() to start website
pub fn start_app() {
    yew::start_app::<App>();
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <h1>{ get_startup_message() }</h1>
    }
}

/// Return Hello, world!
///
/// # Examples
/// ```
/// assert_eq!("Hello, world!", webui::get_startup_message());
/// ```
pub fn get_startup_message() -> &'static str {
    "Hello, world!"
}
