use crate::prelude::*;

pub mod myfi_sdauth;

pub(crate) use myfi_sdauth::*;

pub async fn get_myfi_page_data(page: &str) -> Option<String> {
    match to_base64(page) {
        Some(page_encoded) => {
            let response = fetch(FetchRequest::new(
                format!("https://api.myfi.ws/data/page/{}", page_encoded),
                FetchMethod::Get,
            ))
            .await;
            if !response.is_ok() {
                return None;
            }
            response.get_result()
        }
        None => None,
    }
}
