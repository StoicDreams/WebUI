use crate::prelude::*;

mod prelude;

/* Define web-worker processes here */

/// Get UUID generated from JavaScript.
/// example
/// ```javascript,ignore
/// let data = await webui.worker.send('get_worker_data', 'key');
/// ```
#[wasm_bindgen]
pub fn new_uuid() -> String {
    get_uuid()
}

/// Get worker data
/// example
/// ```javascript,ignore
/// let data = await webui.worker.send('get_worker_data', 'key');
/// ```
#[wasm_bindgen]
pub fn get_worker_data(key: String) -> String {
    get_global_data(key)
}

/// Set worker data
/// example
/// ```javascript,ignore
/// await webui.worker.send('set_worker_data', 'key', 'value');
/// ```
#[wasm_bindgen]
pub fn set_worker_data(key: String, value: String) {
    set_global_data(key, value);
}
