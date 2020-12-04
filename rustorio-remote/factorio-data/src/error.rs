use thiserror::Error;


fn start_of_string(s: &str) -> &str {
    &s[0..10]
}


#[derive(Debug, Error)]
pub enum Error {
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("Invalid blueprint string: (starts with) {}", start_of_string(.0))]
    InvalidBlueprintString(String),
    #[error("Base64 decode error: {0}")]
    Base64Decode(#[from] base64::DecodeError),
    #[error("Invalid signal representation: {0:?}")]
    InvalidSignal(String),
    #[error("Parse error: {0}")]
    ParseDisplay(#[from] parse_display::ParseError),
    #[error("ParseIntError: {0}")]
    ParseIntError(#[from] std::num::ParseIntError),
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
}