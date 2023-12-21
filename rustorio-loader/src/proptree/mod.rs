pub mod de;
pub mod value;

use std::fmt::Display;

use serde::Deserialize;

pub use self::{
    de::Deserializer,
    value::Value,
};

pub fn from_slice<'a, T>(input: &'a [u8]) -> Result<T, Error>
where
    T: Deserialize<'a>,
{
    let mut deserializer = Deserializer::from_slice(input);
    let x = T::deserialize(&mut deserializer)?;

    let rest = deserializer.into_slice();
    if rest.is_empty() {
        Ok(x)
    }
    else {
        Err(Error::TrailingBytes)
    }
}

use self::de::InvalidPropertyType;

#[derive(Debug, thiserror::Error)]
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

impl serde::de::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Self::Custom(format!("{}", msg))
    }
}
