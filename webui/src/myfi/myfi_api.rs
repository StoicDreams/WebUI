mod request;
mod response;

use std::sync::Arc;

use crate::prelude::*;
use request::*;
use response::*;
use uuid::timestamp::context;
use web_sys::Request;

const MYFI_ROOT_AUTH: &str = "auth";
const MYFI_URL_MYINFO: &str = "myinfo";
const MYFI_URL_SIGNOUT: &str = "signout";

pub(crate) async fn myfi_get_my_info(user_state: UseStateHandle<Option<MyFiUser>>) {
    let user_state = user_state.clone();
    let url = format!("https://{}.myfi.ws/{}", MYFI_ROOT_AUTH, MYFI_URL_MYINFO);
    let response = fetch_cors(FetchRequest::new(url.to_string(), FetchMethod::Get)).await;
    if response.is_ok() {
        if let Some(result) = response.get_result() {
            if let Ok(user) = serde_json::from_str::<MyFiUser>(&result) {
                user_state.clone().set(Some(user));
                return;
            }
        }
    }
    user_state.clone().set(Some(MyFiUser::default()));
}

pub(crate) fn myfi_sign_out(contexts: Contexts) {
    let user_state = contexts.clone().user;
    let url = format!("https://{}.myfi.ws/{}", MYFI_ROOT_AUTH, MYFI_URL_SIGNOUT);
    let contexts = contexts.clone();
    spawn_async!({
        _ = fetch_cors(FetchRequest::new(url.to_string(), FetchMethod::Get)).await;
        user_state.clone().set(Some(MyFiUser::default()));
        alert!(contexts, "Success", "You have successfully signed out.");
    });
}
