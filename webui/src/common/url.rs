use js_sys::decode_uri;

use crate::prelude::*;

/// Get the value from a single url query data key.
///
/// If url is None, then current browser URL will be used.
pub fn query_url(name: &str, url: Option<String>) -> Option<String> {
    let query_data = match url {
        Some(url) => get_query_data_from_url(&url),
        None => get_query_data_from_path(),
    };
    if let Some(query_data) = query_data {
        return query_data
            .iter()
            .find(|(key, _)| *key == name)
            .map(|(_, value)| match decode_uri(value) {
                Ok(js_value) => format!("{}", js_value),
                Err(_) => value.to_string(),
            });
    }
    None
}

/// Get a HashMap of all url query data from the current page.
pub fn get_query_data_from_path() -> Option<HashMap<String, String>> {
    let path = get_full_path();
    get_query_data_from_url(&path)
}

pub fn get_query_data_from_url(url: &str) -> Option<HashMap<String, String>> {
    match url.find('?') {
        Some(index) => {
            let query = &url[index + 1..];
            let mut query_data = HashMap::new();
            for pair in query.split('&') {
                let mut pair = pair.split('=');
                let key = pair.next().unwrap_or_default();
                let value = pair.next().unwrap_or_default();
                if query_data.contains_key(key) {
                    query_data.insert(key.to_string(), format!("{},{}", query_data[key], value));
                } else {
                    query_data.insert(key.to_string(), value.to_string());
                }
            }
            Some(query_data)
        }
        None => None,
    }
}
