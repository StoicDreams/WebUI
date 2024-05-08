use crate::prelude::*;

#[derive(Debug, Serialize)]
struct EventLog {
    event: String,
    data: Option<String>,
}

pub async fn log_myfi_event(event: String, data: Option<String>) {
    let event_log = EventLog { event, data };
    if let Ok(post_body) = to_json(&event_log) {
        _ = fetch(FetchRequest::new(
            "https://api.myfi.ws/data/event".to_string(),
            FetchMethod::Post(post_body.to_string()),
        ))
        .await;
    }
}
