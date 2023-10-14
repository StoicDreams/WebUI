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

pub(crate) async fn myfi_get_my_info(user_state: UseStateHandle<Option<MyFiUser>>) -> bool {
    let user_state = user_state.clone();
    let url = format!("https://{}.myfi.ws/{}", MYFI_ROOT_AUTH, MYFI_URL_MYINFO);
    let response = fetch_cors(FetchRequest::new(url.to_string(), FetchMethod::Get)).await;
    if response.is_ok() {
        if let Some(result) = response.get_result() {
            if let Ok(user) = serde_json::from_str::<MyFiUser>(&result) {
                let roles = user.roles.to_owned();
                user_state.clone().set(Some(user));
                return roles > 0;
            }
        }
    }
    user_state.clone().set(Some(MyFiUser::default()));
    false
}

pub enum SignoutScope {
    ThisWebsite,
    ThisBrowser,
    AllDevices,
}

pub fn myfi_sign_out(contexts: Contexts, scope: SignoutScope) {
    let user_state = contexts.clone().user;
    let scope = match scope {
        SignoutScope::ThisWebsite => "site",
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
        myfi_get_my_info(user_state).await;
    });
}
