use js_sys::Array;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

/// Check if given path matches the current browser path
pub fn is_current_path(path: String) -> bool {
    get_path().to_lowercase() == path.to_lowercase()
}

/// Macro for quickly logging data to the browser's console.log
///
/// ```rust,ignore
/// use webui::jslog;
///
/// jslog!("hello {}", "world");
/// jslog!("A B C {} {} {}", 1, 2, 3);
/// ```
#[macro_export]
macro_rules! jslog {
    ( $($x:expr ),* ) => {
        $crate::interop::log(format!($($x),*));
    };
}

#[wasm_bindgen(module = "/src/static_files/js/webui_interop.js")]
extern "C" {
    /// Handler for opening external links
    #[wasm_bindgen]
    pub fn open_external_link(href: &str, target: &str);

    /// General run method
    #[wasm_bindgen]
    pub fn run_method(method: &str, args: &JsValue) -> JsValue;

    #[wasm_bindgen]
    pub fn show_alert(message: &str, variant: &str);

    /// Get host from window
    #[wasm_bindgen]
    pub fn get_host() -> String;

    /// Get origin from window
    #[wasm_bindgen]
    pub fn get_origin() -> String;

    /// Get url page path with query data
    ///
    /// Returned path always starts with forward slash '/'.
    #[wasm_bindgen]
    pub fn get_full_path() -> String;

    /// Log a message to the browser console.
    #[wasm_bindgen]
    pub fn log(message: String);

    /// Generate a uuid from javasscript
    ///
    /// Uses crypto.randomUUID when available, falling back to manually creating one from Math.random() when not.
    #[wasm_bindgen]
    pub fn get_uuid() -> String;

    #[wasm_bindgen]
    pub fn get_global_data(key: String) -> String;

    #[wasm_bindgen]
    pub fn set_global_data(key: String, value: String);

    #[wasm_bindgen]
    pub(crate) async fn webui_fetch(url: String, options: String, useCors: bool) -> JsValue;

    /// Get url page path
    ///
    /// Returned path always starts with forward slash '/'.
    /// Will not include any query data.
    #[wasm_bindgen]
    pub fn get_path() -> String;
}
