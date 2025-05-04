use js_sys::{Array, Function};

use crate::prelude::*;

pub fn newid() -> Uuid {
    Uuid::from_str(&interop::get_uuid()).unwrap()
}

pub fn to_json<T: Serialize>(value: &T) -> Result<String, WebUIError> {
    let json = serde_json::ser::to_string_pretty(value)?;
    Ok(json)
}

pub fn from_json<T: DeserializeOwned>(value: &str) -> Result<T, WebUIError> {
    let instance = serde_json::from_str(value)?;
    Ok(instance)
}

pub fn parse_uuid(value: &str) -> Result<Uuid, WebUIError> {
    match Uuid::parse_str(value) {
        Ok(uuid) => Ok(uuid),
        Err(_) => Err(WebUIError::Invalid(String::from("Invalid UUID"))),
    }
}
