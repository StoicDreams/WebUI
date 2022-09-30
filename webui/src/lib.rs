//! # Web UI
//! 
//! `webui` is a website framework for building webassembly SPA websites quickly and easily.
//! Development is just getting started, so we do not recommend using at this point for anything more than experimenting.

/// Return Hello, world!
/// 
/// # Examples
/// ```
/// assert_eq!("Hello, world!", webui::get_startup_message());
/// ```
pub fn get_startup_message() -> &'static str {
    return "Hello, world!";
}
