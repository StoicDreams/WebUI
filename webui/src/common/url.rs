use crate::prelude::*;

/// Get the value from a single url query data key.
pub fn query_url(name: &str) -> Option<String> {
    if let Some(query_data) = get_query_data_from_path() {
        return query_data
            .iter()
            .find(|(key, _)| *key == name)
            .map(|(_, value)| value.to_string());
    }
    None
}

/// Get a HashMap of all url query data from the current page.
pub fn get_query_data_from_path() -> Option<HashMap<String, String>> {
    let path = get_full_path();
    match path.find('?') {
        Some(index) => {
            let query = &path[index + 1..];
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
