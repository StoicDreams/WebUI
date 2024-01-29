use crate::prelude::*;
use std::collections::HashMap;
use std::sync::Mutex;

/// Container for global data persisted to memory only
///
/// Data is stored in memory. Browser refreshing or page loads will always start with empty data.
/// For persistent memory stored in the browser, look at `webui::get_user_storage_data` and `webui::set_user_storage_data`
pub struct GlobalData;

impl GlobalData {
    /// Save data to global storage.
    ///
    /// Passed value is serialized into json and stored in memory.
    /// Success will return Ok(json)
    pub fn set_data<T>(key: &str, value: T) -> Result<String, WebUIError>
    where
        T: serde::Serialize,
    {
        let result = serde_json::to_string(&value);
        match result {
            Ok(json) => {
                // set_global_data(String::from(key), json.to_string());
                match HASHED_DATA.lock() {
                    Ok(mut table) => {
                        let _ = table.insert(key.to_string(), json.to_owned());
                    }
                    Err(error) => {
                        jslog!("Error in GlobalData.set_data: {:?}", error);
                    }
                }
                Ok(json)
            }
            Err(error) => {
                jslog!("Error in GlobalData.set_data: {:?}", error);
                Err(WebUIError::LockError(String::from(
                    "Unable to retrieve lock to set global data",
                )))
            }
        }
    }

    /// Get data stored in memory from set_data method
    pub fn get_data_or<T>(key: &str, default_handler: fn() -> T) -> T
    where
        T: for<'a> serde::Deserialize<'a>,
    {
        // let json = get_global_data(String::from(key));
        match HASHED_DATA.lock() {
            Ok(table) => {
                let json = match table.get(key) {
                    Some(value) => value.to_string(),
                    None => String::default(),
                };
                if json.is_empty() {
                    return (default_handler)();
                }
                let data = serde_json::from_str::<T>(&json);
                match data {
                    Ok(value) => value,
                    Err(_) => (default_handler)(),
                }
            }
            Err(_) => (default_handler)(),
        }
    }

    /// Get data stored in memory from set_data method
    pub fn get_data<T>(key: &str) -> Result<T, WebUIError>
    where
        T: for<'a> serde::Deserialize<'a>,
    {
        // let json = get_global_data(String::from(key));
        match HASHED_DATA.lock() {
            Ok(table) => {
                let json = match table.get(key) {
                    Some(value) => value.to_string(),
                    None => String::default(),
                };
                if json.is_empty() {
                    return Err(WebUIError::MissingData);
                }
                let data = serde_json::from_str::<T>(&json);
                match data {
                    Ok(value) => Ok(value),
                    Err(error) => {
                        jslog!("Error in GlobalData.get_data: {:?}", error);
                        Err(WebUIError::JsonParseError)
                    }
                }
            }
            Err(_) => Err(WebUIError::LockError(String::from(
                "Unable to retrieve lock to get global data",
            ))),
        }
    }
}

lazy_static! {
    static ref HASHED_DATA: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
}
