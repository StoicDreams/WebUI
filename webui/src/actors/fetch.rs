use crate::prelude::*;

#[derive(Debug, serde::Serialize, Clone, PartialEq)]
pub enum FetchMethod {
    Get,
    Patch(String),
    Post(String),
    Put(String),
    Delete,
}

impl FetchMethod {
    pub fn to_http_method(&self) -> String {
        match self {
            FetchMethod::Get => "GET".to_string(),
            FetchMethod::Patch(_) => "PATCH".to_string(),
            FetchMethod::Post(_) => "POST".to_string(),
            FetchMethod::Put(_) => "PUT".to_string(),
            FetchMethod::Delete => "DELETE".to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FetchRequest {
    url: String,
    method: FetchMethod,
    headers: HashMap<String, String>,
    use_cors: bool,
}

impl FetchRequest {
    pub fn new(url: String, method: FetchMethod) -> Self {
        Self {
            url,
            method,
            headers: HashMap::new(),
            use_cors: false,
        }
    }
    pub fn new_cors(url: String, method: FetchMethod) -> Self {
        Self {
            url,
            method,
            headers: HashMap::new(),
            use_cors: true,
        }
    }
    pub fn add_header(&mut self, key: &str, value: &str) -> &mut Self {
        self.headers.insert(String::from(key), String::from(value));
        self
    }
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct FetchResponse {
    headers: HashMap<String, String>,
    status: u16,
    body: Option<String>,
}

impl FetchResponse {
    pub fn is_ok(&self) -> bool {
        self.status >= 200 && self.status < 300
    }
    pub fn get_result(&self) -> Option<String> {
        self.body.to_owned()
    }
    pub fn ok(status: u16, body: String) -> Self {
        FetchResponse {
            headers: HashMap::new(),
            status,
            body: Some(body),
        }
    }
    pub fn error() -> Self {
        FetchResponse {
            headers: HashMap::new(),
            status: 500,
            body: None,
        }
    }
}

fn build_url(url: &str) -> String {
    if url.is_empty() {
        return url.to_string();
    }
    if url.starts_with("http") {
        return url.to_string();
    }
    if url.starts_with('/') {
        return format!("{}{}", interop::get_origin(), url);
    }
    format!("{}/{}", interop::get_full_path(), url)
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct FetchOptions {
    method: String,
    body: Option<String>,
}

/// Fetch data from server with CORS policy enabled
pub async fn fetch_cors(request: FetchRequest) -> FetchResponse {
    let request = FetchRequest::new_cors(request.url, request.method);
    fetch(request).await
}

/// Fetch data from server
pub async fn fetch(request: FetchRequest) -> FetchResponse {
    let mut options = FetchOptions {
        method: request.method.to_http_method(),
        body: None,
    };
    match request.method {
        FetchMethod::Get => (),
        FetchMethod::Patch(data) => options.body = Some(data),
        FetchMethod::Post(data) => options.body = Some(data),
        FetchMethod::Put(data) => options.body = Some(data),
        FetchMethod::Delete => (),
    };

    let json = serde_json::to_string(&options).unwrap();
    let url = build_url(&request.url);
    let result = webui_fetch(url, json, request.use_cors).await;
    if let Some(result) = result.as_string() {
        if let Ok(result) = serde_json::from_str::<FetchResponse>(&result) {
            return result;
        }
    }
    FetchResponse::error()
}
