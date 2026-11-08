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

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct Sample {
        name: String,
        count: u32,
    }

    #[test]
    fn json_round_trips() {
        let sample = Sample {
            name: String::from("a \"quoted\" name"),
            count: 3,
        };
        let json = to_json(&sample).unwrap();
        assert!(json.contains('\n'), "to_json is pretty printed");
        assert_eq!(from_json::<Sample>(&json).unwrap(), sample);
    }

    #[test]
    fn json_that_does_not_fit_is_an_error() {
        assert!(from_json::<Sample>("{\"name\": 5}").is_err());
        assert!(from_json::<Sample>("not json").is_err());
    }

    #[test]
    fn parse_uuid_accepts_uuids_and_refuses_other_text() {
        let text = "67e55044-10b1-426f-9247-bb680e5fe0c8";
        assert_eq!(parse_uuid(text).unwrap().to_string(), text);
        assert!(parse_uuid("").is_err());
        assert!(parse_uuid("67e55044-10b1-426f-9247").is_err());
    }
}
