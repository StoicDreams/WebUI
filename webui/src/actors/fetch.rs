use crate::*;
use reqwest::*;

#[derive(Debug, serde::Serialize)]
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

#[derive(Debug)]
pub struct FetchRequest {
    url: String,
    method: FetchMethod,
    headers: header::HeaderMap,
}

impl FetchRequest {
    pub fn new(url: String, method: FetchMethod) -> Self {
        Self {
            url,
            method,
            headers: header::HeaderMap::new(),
        }
    }
}

#[derive(Debug)]
pub struct FetchResponse {
    status: u16,
    result: Option<String>,
}

impl FetchResponse {
    pub fn is_ok(&self) -> bool {
        self.status >= 200 && self.status < 300
    }
    pub fn get_result(&self) -> Option<String> {
        self.result.to_owned()
    }
    pub fn ok(status: u16, body: String) -> Self {
        FetchResponse {
            status,
            result: Some(body),
        }
    }
    pub fn error() -> Self {
        FetchResponse {
            status: 500,
            result: None,
        }
    }
}
trait Block {
    fn wait(self) -> <Self as futures::Future>::Output
    where
        Self: Sized,
        Self: futures::Future,
    {
        futures::executor::block_on(self)
    }
}

impl<F, T> Block for F where F: futures::Future<Output = T> {}

fn build_url(url: &str) -> String {
    if url.is_empty() {
        return url.to_string();
    }
    if url.starts_with("http") {
        return url.to_string();
    }
    if url.starts_with("/") {
        return format!("{}{}", interop::get_origin(), url);
    }
    format!("{}/{}", interop::get_full_path(), url)
}

pub async fn fetch(request: FetchRequest) -> FetchResponse {
    let client = reqwest::Client::builder()
        .default_headers(request.headers)
        .build();
    let url = build_url(&request.url);
    match client {
        Ok(client) => match request.method {
            FetchMethod::Get => {
                let result = client.get(url).send().await;
                handle_result(result).await
            }
            FetchMethod::Patch(data) => {
                let result = client.patch(url).body(data).send().await;
                handle_result(result).await
            }
            FetchMethod::Post(data) => {
                let result = client.post(url).body(data).send().await;
                handle_result(result).await
            }
            FetchMethod::Put(data) => {
                let result = client.put(url).body(data).send().await;
                handle_result(result).await
            }
            FetchMethod::Delete => {
                let result = client.delete(url).send().await;
                handle_result(result).await
            }
        },
        Err(_error) => FetchResponse::error(),
    }
}

async fn handle_result(result: reqwest::Result<reqwest::Response>) -> FetchResponse {
    match result {
        Ok(response) => {
            let status = response.status().as_u16();
            let body = response.text().await.unwrap_or_default();
            FetchResponse::ok(status, body)
        }
        Err(_err) => FetchResponse::error(),
    }
}
