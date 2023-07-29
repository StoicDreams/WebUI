use crate::prelude::*;
use std::{env::VarError, fmt::Display, string::FromUtf8Error};

#[derive(Debug)]
pub enum WebUIError {
    Invalid(String),
    MissingData,
    JsonParseError,
    JsonSerializeError,
    LockError(String),
    Error(String),
}

impl std::error::Error for WebUIError {}

impl std::fmt::Display for WebUIError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WebUIError::Invalid(val) => write!(f, "Invalid: {}", val),
            WebUIError::MissingData => write!(f, "Missing Data"),
            WebUIError::JsonParseError => write!(f, "JSON Parse Error"),
            WebUIError::JsonSerializeError => write!(f, "JSON Serialization Error"),
            WebUIError::LockError(val) => write!(f, "Lock Error: {}", val),
            WebUIError::Error(val) => write!(f, "Error: {}", val),
        }
    }
}

impl From<std::fmt::Error> for WebUIError {
    fn from(value: std::fmt::Error) -> Self {
        WebUIError::Error(value.to_string())
    }
}

impl From<std::option::Option<std::convert::Infallible>> for WebUIError {
    fn from(value: std::option::Option<std::convert::Infallible>) -> Self {
        WebUIError::Error(format!("{:?}", value))
    }
}

impl From<serde_json::Error> for WebUIError {
    fn from(value: serde_json::Error) -> Self {
        match value.classify() {
            serde_json::error::Category::Io => WebUIError::Error(String::from("IO Error")),
            serde_json::error::Category::Syntax => WebUIError::JsonParseError,
            serde_json::error::Category::Data => WebUIError::JsonParseError,
            serde_json::error::Category::Eof => WebUIError::JsonParseError,
        }
    }
}
