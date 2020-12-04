pub mod value;

#[cfg(feature="nalgebra")]
pub mod nalgebra;
#[cfg(feature="palette")]
mod palette;


use std::{
    collections::{HashMap, BTreeMap},
    hash::Hash,
    cmp::Eq,
    convert::TryFrom,
    num::TryFromIntError,
    fmt::Display,
};

pub use mlua::{Value, Table};
use thiserror::Error;


pub fn to_option<T: FromLuaValue>(value: Value) -> Result<Option<T>, Error> {
    Ok(match value {
        Value::Nil => None,
        value => Some(T::from_lua_value(value)?),
    })
}

pub fn to_result<T, E, F>(value: Value, f: F) -> Result<T, Error>
    where
        T: FromLuaValue,
        Error: From<E>,
        F: FnMut() -> E
{
    Ok(to_option(value)?.ok_or_else(f)?)
}


#[derive(Debug, Error)]
pub enum Error {
    #[error("Can't convert Lua value: {0}")]
    Unexpected(String),

    #[error("Can't convert Lua value: {0}")]
    Inconvertible(String),

    #[error("Lua error: {0}")]
    Lua(#[from] mlua::Error),

    #[error("Can't convert number: {0}")]
    NumConvert(#[from] TryFromIntError),

    #[error("Duplicate key in table")]
    // TODO: Put key into here (maybe we should first move rustorio_core::lua_utils::value to here to have a dynamic
    // and owned "LuaValue".
    DuplicateKey,

    #[error("Infallible")]
    Infallible(#[from] std::convert::Infallible),

    #[error("Missing field: {0}")]
    MissingField(String),

    #[error("{0}")]
    Other(String)
}

impl Error {
    pub fn inconvertible(x: Value) -> Self {
        Self::Inconvertible(format!("{:?}", x))
    }

    pub fn unexpected(x: Value) -> Self {
        Self::Unexpected(format!("{:?}", x))
    }

    pub fn other<T: Display>(x: T) -> Self {
        Self::Other(x.to_string())
    }

    pub fn missing_field<T: Display>(field_name: T) -> Self {
        Self::MissingField(field_name.to_string())
    }
}

pub trait FromLuaValue: Sized {
    fn from_lua_value(value: Value) -> Result<Self, Error>;
}

pub trait FromLuaTable: Sized {
    fn from_lua_table(table: Table) -> Result<Self, Error>;
}


macro_rules! impl_with_try_from {
    ($out:ident, $in:ident) => {
        impl FromLuaValue for $out {
            fn from_lua_value(value: Value) -> Result<Self, Error> {
                match value {
                    Value::$in(x) => Ok(TryFrom::try_from(x)?),
                    x => Err(Error::unexpected(x)),
                }
            }
        }
    }
}

impl_with_try_from!(i8, Integer);
impl_with_try_from!(u8, Integer);
impl_with_try_from!(i16, Integer);
impl_with_try_from!(u16, Integer);
impl_with_try_from!(i32, Integer);
impl_with_try_from!(u32, Integer);
impl_with_try_from!(i64, Integer);
impl_with_try_from!(u64, Integer);
//impl_with_try_from!(f32, Number);
//impl_with_try_from!(f64, Number);

impl FromLuaValue for f32 {
    fn from_lua_value(value: Value) -> Result<Self, Error> {
        Ok(f64::from_lua_value(value)? as f32)
    }
}

impl FromLuaValue for f64 {
    fn from_lua_value(value: Value) -> Result<Self, Error> {
        match value {
            Value::Number(x) => Ok(x),
            Value::Integer(x) => Ok(x as f64),
            x => Err(Error::unexpected(x)),
        }
    }
}

impl FromLuaValue for bool {
    fn from_lua_value(value: Value) -> Result<Self, Error> {
        match value {
            Value::Boolean(x) => Ok(x),
            x => Err(Error::unexpected(x)),
        }
    }
}

impl FromLuaValue for () {
    fn from_lua_value(value: Value) -> Result<Self, Error> {
        match value {
            Value::Nil => Ok(()),
            x => Err(Error::unexpected(x))
        }
    }
}

impl FromLuaValue for String {
    fn from_lua_value(value: Value) -> Result<Self, Error> {
        match value {
            Value::String(x) => Ok(x.to_str()?.to_owned()),
            x => Err(Error::unexpected(x))
        }
    }
}


impl<T: FromLuaTable> FromLuaValue for T {
    fn from_lua_value(value: Value) -> Result<Self, Error> {
        match value {
            Value::Table(table) => T::from_lua_table(table),
            x => Err(Error::unexpected(x)),
        }
    }
}

impl<T: FromLuaValue> FromLuaTable for Vec<T> {
    fn from_lua_table(table: Table) -> Result<Self, Error> {
        let mut v = vec![];
        let mut i = 1;

        loop {
            let x = table.get(Value::Integer(i))?;
            match x {
                Value::Nil => break Ok(v),
                x => v.push(T::from_lua_value(x)?),
            }
            i += 1;
        }
    }
}

macro_rules! impl_for_map {
    ($out:ident; $($trait_bounds:ident),*) => {
        impl<K: FromLuaValue $(+ $trait_bounds)*, V: FromLuaValue> FromLuaTable for $out<K, V> {
            fn from_lua_table(table: Table) -> Result<Self, Error> {
                let mut m = $out::new();
                for r in table.pairs::<mlua::Value, mlua::Value>() {
                    let (k, v) = r?;

                    let k = K::from_lua_value(k)?;
                    let v = V::from_lua_value(v)?;

                    if m.insert(k, v).is_some() {
                        return Err(Error::DuplicateKey);
                    }
                }
                Ok(m)
            }
        }
    }
}

impl_for_map!(HashMap; Hash, Eq);
impl_for_map!(BTreeMap; Ord);

impl<T: FromLuaValue> FromLuaValue for Option<T> {
    fn from_lua_value(value: Value) -> Result<Self, Error> {
        match value {
            Value::Nil => Ok(None),
            value => Ok(Some(T::from_lua_value(value)?))
        }
    }
}
