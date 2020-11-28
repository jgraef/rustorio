use std::fmt::Display;

use serde::de;
use thiserror::Error;

use crate::de::InvalidPropertyType;

#[derive(Debug, Error)]
pub enum Error {
    #[error("{0}")]
    Custom(String),

    #[error("Unexpected end")]
    UnexpectedEnd,

    #[error("{0}")]
    InvalidPropertyType(#[from] InvalidPropertyType),

    #[error("UTF-8 error: {0}")]
    Utf8(#[from] std::str::Utf8Error),

    #[error("Deserialization didn't consume the input fully")]
    TrailingBytes,

    #[error("Keys must be strings")]
    KeyMustBeString,
}

impl de::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Self::Custom(format!("{}", msg))
    }
}
