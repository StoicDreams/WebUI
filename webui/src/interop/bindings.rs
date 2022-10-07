use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/src/static_files/js/webui_interop.js")]
extern "C" {
    #[wasm_bindgen]
    pub fn get_now_date() -> String;
}
