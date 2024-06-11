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

    /// Set the document/page title
    #[wasm_bindgen]
    pub fn set_title(title: &str);

    /// Push path to url
    #[wasm_bindgen]
    pub fn push_state(path: &str);

    /// Set page transition duration
    ///
    /// Expecting value in milliseconds from 1 to 1000
    #[wasm_bindgen]
    pub fn set_page_transition_duration(value: u16);

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

    /// Run to flag user acceptance of persisting data across browser sessions
    ///
    /// This will store any user data in the browsers' localStorage container.
    /// This will clear any data stored in sessionStorage.
    #[wasm_bindgen]
    pub fn user_accepts_local_storage();

    /// Run to flag user acceptance of persisting data for the current tab/window session
    ///
    /// This will store any user data in the browsers' sessionStorage container.
    /// This will clear any data stored in localStorage.
    #[wasm_bindgen]
    pub fn user_accepts_session_storage();

    /// Run to flag user rejection of persisting browser data.
    ///
    /// This will clear any data in localStorage and sessionStorage.
    #[wasm_bindgen]
    pub fn user_rejects_cached_storage();

    /// Set user data to store in appropriate container
    ///
    /// All data is stored and retrieved in a memory storage.
    /// All data is expected to be encrypted|serialized for storage prior to passing to this method.
    /// When session or local storage is enabled, data is also backed up to the appropriate container, and loaded into memory storage on page loads.
    #[wasm_bindgen]
    pub fn set_user_storage_data(key: String, value: String);

    /// Get user data to store in appropriate container
    ///
    /// If data does not exist then an empty string will be returned.
    /// All data is stored and retrieved in a memory storage.
    /// When session or local storage is enabled, data is also backed up to the appropriate container, and loaded into memory storage on page loads.
    #[wasm_bindgen]
    pub fn get_user_storage_data(key: String) -> String;

    #[wasm_bindgen]
    pub fn get_global_data(key: String) -> String;

    #[wasm_bindgen]
    pub fn set_global_data(key: String, value: String);

    #[wasm_bindgen]
    pub(crate) async fn webui_fetch(url: String, options: String, useCors: bool) -> JsValue;

    #[wasm_bindgen]
    pub fn app_has_classes(classes: String) -> bool;

    #[wasm_bindgen]
    pub fn add_app_class(classes: String);

    #[wasm_bindgen]
    pub fn update_app_classes(add_classes: String, remove_classes: String);

    #[wasm_bindgen]
    pub fn remove_app_class(classes: String);

    /// Get url page path
    ///
    /// Returned path always starts with forward slash '/'.
    /// Will not include any query data.
    #[wasm_bindgen]
    pub fn get_path() -> String;
}
