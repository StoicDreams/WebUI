use crate::prelude::*;

pub async fn get_page_data(page: &str) -> Option<String> {
    match to_base64(page) {
        Some(page_encoded) => {
            let response = fetch(FetchRequest::new(
                format!("https://data.myfi.ws/page/{}", page_encoded),
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
