mod request;
mod response;

use std::sync::Arc;

use crate::prelude::*;
use request::*;
use response::*;
use uuid::timestamp::context;
use web_sys::Request;

const MYFI_ROOT_AUTH: &str = "auth";
const MYFI_URL_SESSION: &str = "session";
const MYFI_URL_MYINFO: &str = "myinfo";
const MYFI_URL_SIGNIN: &str = "signin";
const MYFI_URL_SIGNOUT: &str = "signout";

pub(crate) async fn myfi_get_session() {
    let url = format!("https://{}.myfi.ws/{}", MYFI_ROOT_AUTH, MYFI_URL_SESSION);
    _ = fetch(FetchRequest::new(url.to_string(), FetchMethod::Get)).await;
}

pub(crate) async fn myfi_get_my_info(user_state: Arc<UseStateHandle<Option<MyFiUser>>>) {
    let user_state = user_state.clone();
    let url = format!("https://{}.myfi.ws/{}", MYFI_ROOT_AUTH, MYFI_URL_MYINFO);
    let response = fetch(FetchRequest::new(url.to_string(), FetchMethod::Get)).await;
    if response.is_ok() {
        if let Some(result) = response.get_result() {
            if let Ok(user) = serde_json::from_str::<MyFiUser>(&result) {
                if user.roles > 0 {
                    user_state.clone().set(Some(user));
                    return;
                }
            }
        }
    }
    user_state.clone().set(None);
}

pub(crate) fn myfi_sign_in(
    contexts: Contexts,
    email: &str,
    password: &str,
    alert_state: UseStateHandle<String>,
    submitting_state: UseStateHandle<bool>,
) {
    let user_state = contexts.clone().user;
    let email = email.to_string();
    let password = password.to_string();
    let url = format!("https://{}.myfi.ws/{}", MYFI_ROOT_AUTH, MYFI_URL_SIGNIN);
    let post_data = HashMap::from([("email", email), ("password", password)]);
    match serde_json::to_string(&post_data) {
        Ok(post_body) => {
            let contexts = contexts.clone();
            let alert_state = alert_state.clone();
            let submitting_state = submitting_state.clone();
            spawn_async!({
                let response = fetch(FetchRequest::new(
                    url.to_string(),
                    FetchMethod::Post(post_body.to_string()),
                ))
                .await;
                if response.is_ok() {
                    if let Some(result) = response.get_result() {
                        if let Ok(user) = serde_json::from_str::<MyFiUser>(&result) {
                            contexts.drawer.clone().set(DrawerMessage::Close);
                            let name = user.display_name.clone();
                            user_state.clone().set(Some(user));
                            alert!(
                                contexts,
                                "Success",
                                format!("Welcome {}, you have successfully signed in.", name)
                            );
                            submitting_state.clone().set(false);
                            return;
                        }
                    }
                    alert_state.clone().set("Unknown error".to_string());
                } else if let Some(result) = response.get_result() {
                    alert_state.clone().set(result.clone());
                } else {
                    alert_state.clone().set("Unknown error".to_string());
                }
                user_state.clone().set(None);
                submitting_state.clone().set(false);
            });
        }
        Err(error) => {
            let message = format!("{}", error);
            alert_state.clone().set(message.clone());
            submitting_state.clone().set(false);
        }
    }
}

pub(crate) fn myfi_sign_out(contexts: Contexts) {
    let user_state = contexts.clone().user;
    let url = format!("https://{}.myfi.ws/{}", MYFI_ROOT_AUTH, MYFI_URL_SIGNOUT);
    let contexts = contexts.clone();
    spawn_async!({
        _ = fetch(FetchRequest::new(url.to_string(), FetchMethod::Get)).await;
        user_state.clone().set(None);
        alert!(contexts, "Success", "You have successfully signed out.");
    });
}
