use wasm_bindgen::prelude::*;

pub fn is_current_path(path: String) -> bool {
    get_path().to_lowercase() == path.to_lowercase()
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
