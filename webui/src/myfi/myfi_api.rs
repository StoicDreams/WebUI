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
thread_local!(static MYFI_APP_KEY: Cell<&'static str> = Cell::new(""));

/// Use this method to set your assigned public MyFi application key.
///
/// This is needed for desktop applications to use MyFi authenticated services.
#[no_mangle]
pub fn set_myfi_app_key(key: &str) {
    MYFI_APP_KEY.with(|a: &Cell<&'static str>| a.set(Box::leak(key.to_string().into_boxed_str())));
}
pub(crate) fn get_myfi_app_key() -> Option<&'static str> {
    let key = MYFI_APP_KEY.with(|a| a.get());
    if key.is_empty() {
        return None;
    }
    Some(key)
}

pub(crate) async fn myfi_get_my_info(
    user_state: UseStateHandle<Option<MyFiUser>>,
    roles_state: UseStateHandle<u32>,
) -> bool {
    let user_state = user_state.clone();
    let url = format!("https://{}.myfi.ws/{}", MYFI_ROOT_AUTH, MYFI_URL_MYINFO);
    let response = fetch_cors(FetchRequest::new(url.to_string(), FetchMethod::Get)).await;
    if response.is_ok() {
        if let Some(result) = response.get_result() {
            if let Ok(user) = serde_json::from_str::<MyFiUser>(&result) {
                let roles = user.roles.to_owned();
                user_state.clone().set(Some(user));
                roles_state.clone().set(roles);
                return roles > 0;
            }
        }
    }
    user_state.clone().set(Some(MyFiUser::default()));
    roles_state.clone().set(0);
    false
}

pub enum SignoutScope {
    ThisApp,
    ThisBrowser,
    AllDevices,
}

pub fn myfi_sign_out(contexts: &Contexts, scope: SignoutScope) {
    let user_state = contexts.clone().user;
    let roles_state = contexts.clone().user_roles;
    let scope = match scope {
        SignoutScope::ThisApp => "app",
        SignoutScope::ThisBrowser => "browser",
        SignoutScope::AllDevices => "all",
    };
    let url = format!("https://{}.myfi.ws/{}", MYFI_ROOT_AUTH, MYFI_URL_SIGNOUT);
    let contexts = contexts.clone();
    let request = FetchRequest::new(url.to_string(), FetchMethod::Get)
        .add_header("x-scope", scope)
        .to_owned();
    spawn_async!({
        _ = fetch_cors(request).await;
        user_state.clone().set(Some(MyFiUser::default()));
        alert!(contexts, "Success", "You have successfully signed out.");
        set_user_storage_data(String::from("stoic_dreams_auth_token"), String::default());
        myfi_get_my_info(user_state, roles_state).await;
    });
}
