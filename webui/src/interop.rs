use wasm_bindgen::prelude::*;

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
    #[wasm_bindgen]
    pub fn get_now_date() -> String;

    #[wasm_bindgen]
    pub fn get_path() -> String;

    #[wasm_bindgen]
    pub fn get_full_path() -> String;

    #[wasm_bindgen]
    pub fn log(message: String);
}
