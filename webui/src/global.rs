use js_sys::Function;

use crate::{interop, prelude::*};

pub fn newid() -> Uuid {
    Uuid::from_str(&interop::get_uuid()).unwrap()
}

pub fn empty_html() -> Html {
    html! {}
}

pub fn get_window() -> web_sys::Window {
    web_sys::window().unwrap()
}

pub fn set_timeout(handler: &Function, milliseconds: i32) -> Result<i32, JsValue> {
    let window = get_window();
    window.set_timeout_with_callback_and_timeout_and_arguments_0(handler, milliseconds)
}
