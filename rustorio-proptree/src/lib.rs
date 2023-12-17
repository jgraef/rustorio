pub mod de;
pub mod error;
pub mod value;

use serde::Deserialize;

pub use crate::{
    de::Deserializer,
    error::Error,
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
        log::error!("Trailing bytes:\n{}", pretty_hex::pretty_hex(&&rest[..32]));
        Err(Error::TrailingBytes)
    }
}

#[cfg(test)]
mod tests {
    use super::{
        from_slice,
        Value,
    };

    #[test]
    fn it_deserializes_mod_settings() {
        pretty_env_logger::init();

        let data = &include_bytes!("../mod-settings.dat")[9..];

        let parsed: Value = from_slice(data).unwrap();

        log::info!("mod-settings.dat: {:#?}", parsed);
    }
}
