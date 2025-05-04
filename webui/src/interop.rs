use js_sys::Array;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

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
}
