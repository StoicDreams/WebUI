use crate::*;
use reqwest::*;

#[derive(Debug, serde::Serialize)]
pub enum FetchMethod {
	Get,
	Post(String),
	Put(String),
	Delete,
}

impl FetchMethod {
	pub fn to_http_method(&self) -> String {
		match self {
			FetchMethod::Get => "GET".to_string(),
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
	pub fn is_ok(self: &Self) -> bool {
		self.status >= 200 && self.status < 300
	}
	pub fn get_result(self: &Self) -> Option<String> {
		self.result.to_owned()
	}
	pub fn error() -> Self {
		FetchResponse {
			status: 500,
			result: None
		}
	}
}
trait Block {
    fn wait(self) -> <Self as futures::Future>::Output
        where Self: Sized, Self: futures::Future
    {
        futures::executor::block_on(self)
    }
}

impl<F,T> Block for F
    where F: futures::Future<Output = T>
{}


pub async fn fetch(request: FetchRequest) -> FetchResponse {
	jslog!("Fetch: {:?}", request);
	let client = reqwest::Client::builder()
		.default_headers(request.headers)
		.build();
	match client {
		Ok(client) => {
			match request.method {
				FetchMethod::Get => {
					let result = client.get(request.url).send().await;
					FetchResponse::error()
				},
				FetchMethod::Post(data) => {
					let result = client.post(request.url).body(data).send().await;
					FetchResponse::error()
				},
				FetchMethod::Put(data) => {
		
					FetchResponse::error()
				},
				FetchMethod::Delete => {
		
					FetchResponse::error()
				}
			}
		},
		Err(error) => {
			FetchResponse::error()
		}
	}
}