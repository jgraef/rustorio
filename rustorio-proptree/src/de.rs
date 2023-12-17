use std::convert::TryFrom;

use byteorder::{
    ByteOrder,
    LittleEndian,
};
use serde::{
    de::{
        self,
        DeserializeSeed,
        Visitor,
    },
    forward_to_deserialize_any,
};

use crate::error::Error;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum PropertyType {
    None,
    Bool,
    Number,
    String,
    List,
    Dictionary,
}

#[derive(Debug, thiserror::Error)]
#[error("Invalid byte for property type: {0}")]
pub struct InvalidPropertyType(u8);

impl TryFrom<u8> for PropertyType {
    type Error = InvalidPropertyType;

    fn try_from(b: u8) -> Result<Self, Self::Error> {
        match b {
            0 => Ok(Self::None),
            1 => Ok(Self::Bool),
            2 => Ok(Self::Number),
            3 => Ok(Self::String),
            4 => Ok(Self::List),
            5 => Ok(Self::Dictionary),
            _ => Err(InvalidPropertyType(b)),
        }
    }
}

impl From<PropertyType> for u8 {
    fn from(ptype: PropertyType) -> u8 {
        match ptype {
            PropertyType::None => 0,
            PropertyType::Bool => 1,
            PropertyType::Number => 2,
            PropertyType::String => 3,
            PropertyType::List => 4,
            PropertyType::Dictionary => 5,
        }
    }
}

pub struct Deserializer<'de> {
    input: &'de [u8],
}

impl<'de> Deserializer<'de> {
    pub fn from_slice(input: &'de [u8]) -> Self {
        Self { input }
    }

    pub fn into_slice(self) -> &'de [u8] {
        self.input
    }

    fn read_bytes(&mut self, n: usize) -> Result<&[u8], Error> {
        log::trace!("Reading {} bytes...", n);
        if n > self.input.len() {
            return Err(Error::UnexpectedEnd);
        }
        let (data, rest) = self.input.split_at(n);
        log::trace!("Read {} bytes: {:?}", n, data);
        self.input = rest;
        Ok(data)
    }

    fn read_property_type(&mut self) -> Result<PropertyType, Error> {
        Ok(PropertyType::try_from(self.read_bytes(1)?[0])?)
    }

    fn read_bool(&mut self) -> Result<bool, Error> {
        Ok(self.read_bytes(1)?[0] == 1)
    }

    fn read_string(&mut self) -> Result<&str, Error> {
        if self.read_bool()? {
            log::trace!("empty string");
            Ok("")
        }
        else {
            let n = self.read_varint()? as usize;
            log::trace!("reading string with length {}", n);
            let data = self.read_bytes(n)?;
            Ok(std::str::from_utf8(data)?)
        }
    }

    fn read_varint(&mut self) -> Result<u32, Error> {
        let b = self.read_bytes(1)?[0];
        if b == 0xff {
            Ok(LittleEndian::read_u32(self.read_bytes(4)?))
        }
        else {
            Ok(b as u32)
        }
    }

    fn read_double(&mut self) -> Result<f64, Error> {
        Ok(LittleEndian::read_f64(self.read_bytes(8)?))
    }

    fn read_u32(&mut self) -> Result<u32, Error> {
        Ok(LittleEndian::read_u32(self.read_bytes(4)?))
    }
}

impl<'de, 'a> de::Deserializer<'de> for &'a mut Deserializer<'de> {
    type Error = crate::error::Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let ptype = self.read_property_type()?;
        let _any = self.read_bool()?;

        log::trace!("deserialize_any: ptype={:?}", ptype);

        match ptype {
            PropertyType::None => visitor.visit_none(),
            PropertyType::Bool => visitor.visit_bool(self.read_bool()?),
            PropertyType::Number => visitor.visit_f64(self.read_double()?),
            PropertyType::String => visitor.visit_str(self.read_string()?),
            PropertyType::List | PropertyType::Dictionary => {
                let n = self.read_u32()?;
                log::trace!("Reading map with {} entries...", n);
                visitor.visit_map(&mut MapAccess {
                    n: n as usize,
                    de: self,
                })
            }
        }
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf option unit unit_struct newtype_struct seq tuple
        tuple_struct map struct enum identifier ignored_any
    }
}

pub struct KeyDeserializer<'de, 'a> {
    inner: &'a mut Deserializer<'de>,
}

impl<'de, 'a> de::Deserializer<'de> for KeyDeserializer<'de, 'a> {
    type Error = crate::error::Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_str(self.inner.read_string()?)
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf option unit unit_struct newtype_struct seq tuple
        tuple_struct map struct enum identifier ignored_any
    }
}

pub struct MapAccess<'de, 'a> {
    n: usize,
    de: &'a mut Deserializer<'de>,
}

impl<'de, 'a> de::MapAccess<'de> for MapAccess<'de, 'a> {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Error>
    where
        K: DeserializeSeed<'de>,
    {
        if self.n > 0 {
            Ok(Some(seed.deserialize(KeyDeserializer {
                inner: &mut *self.de,
            })?))
        }
        else {
            Ok(None)
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Error>
    where
        V: DeserializeSeed<'de>,
    {
        self.n -= 1;
        Ok(seed.deserialize(&mut *self.de)?)
    }
}
