#[derive(Debug)]
pub enum WebUIError {
    MissingData,
    JsonParseError,
    JsonSerializeError,
}

impl std::error::Error for WebUIError {}

impl std::fmt::Display for WebUIError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WebUIError::MissingData => write!(f, "Missing Data"),
            WebUIError::JsonParseError => write!(f, "JSON Parse Error"),
            WebUIError::JsonSerializeError => write!(f, "JSON Serialization Error"),
        }
    }
}
