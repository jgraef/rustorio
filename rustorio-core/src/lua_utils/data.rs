use std::{
    collections::btree_map::{BTreeMap, IntoIter, Iter},
    convert::{TryFrom, TryInto},
    fmt::{self, Debug, Formatter},
    iter::FromIterator,
};

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Can't convert Lua value: {0}")]
    Inconvertible(String),

    #[error("Lua error: {0}")]
    Lua(#[from] mlua::Error),
}

impl Error {
    fn inconvertible(x: mlua::Value) -> Self {
        Self::Inconvertible(format!("{:?}", x))
    }
}


#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum Type {
    Nil,
    Boolean,
    Integer,
    Number,
    String,
    Table,
}

impl Type {
    pub fn can_be_key(&self) -> bool {
        match self {
            Type::Number | Type::Table => false,
            _ => true,
        }
    }
}


#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Key {
    Nil,
    Boolean(bool),
    Integer(i64),
    String(String),
}

impl Key {
    pub fn ty(&self) -> Type {
        match self {
            Key::Nil => Type::Nil,
            Key::Boolean(_) => Type::Boolean,
            Key::Integer(_) => Type::Integer,
            Key::String(_) => Type::String,
        }
    }
}

impl From<bool> for Key {
    fn from(x: bool) -> Self {
        Self::Boolean(x)
    }
}

impl From<i64> for Key {
    fn from(x: i64) -> Self {
        Self::Integer(x)
    }
}

impl From<String> for Key {
    fn from(x: String) -> Self {
        Self::String(x)
    }
}

impl TryFrom<mlua::Value<'_>> for Key {
    type Error = Error;

    fn try_from(value: mlua::Value) -> Result<Self, Self::Error> {
        match value {
            mlua::Value::Nil => Ok(Key::Nil),
            mlua::Value::Boolean(b) => Ok(Key::Boolean(b)),
            mlua::Value::Integer(i) => Ok(Key::Integer(i)),
            mlua::Value::String(s) => Ok(Key::String(s.to_str()?.to_owned())),
            x => Err(Error::inconvertible(x)),
        }
    }
}

impl Debug for Key {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Key::Nil => write!(f, "nil"),
            Key::Boolean(x) => x.fmt(f),
            Key::Integer(x) => x.fmt(f),
            Key::String(x) => x.fmt(f),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Table(BTreeMap<Key, Value>);

impl Table {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn iter(&self) -> Iter<Key, Value> {
        self.0.iter()
    }

    fn get(&self, k: &Key) -> Option<&Value> {
        self.0.get(k)
    }

    fn insert(&mut self, k: Key, v: Value) -> Option<Value> {
        self.0.insert(k, v)
    }
}

impl IntoIterator for Table {
    type Item = (Key, Value);
    type IntoIter = IntoIter<Key, Value>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl FromIterator<(Key, Value)> for Table {
    fn from_iter<T: IntoIterator<Item = (Key, Value)>>(iter: T) -> Self {
        Table(BTreeMap::from_iter(iter))
    }
}

impl AsRef<BTreeMap<Key, Value>> for Table {
    fn as_ref(&self) -> &BTreeMap<Key, Value> {
        &self.0
    }
}

impl AsMut<BTreeMap<Key, Value>> for Table {
    fn as_mut(&mut self) -> &mut BTreeMap<Key, Value> {
        &mut self.0
    }
}

impl From<Table> for BTreeMap<Key, Value> {
    fn from(t: Table) -> Self {
        t.0
    }
}

impl From<BTreeMap<Key, Value>> for Table {
    fn from(t: BTreeMap<Key, Value>) -> Self {
        Table(t)
    }
}

impl TryFrom<mlua::Table<'_>> for Table {
    type Error = Error;

    fn try_from(table: mlua::Table) -> Result<Self, Self::Error> {
        let mut t = BTreeMap::new();

        for r in table.pairs::<mlua::Value, mlua::Value>() {
            let (k, v) = r?;

            t.insert(k.try_into()?, v.try_into()?);
        }

        Ok(Table(t))
    }
}

impl Debug for Table {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut map = f.debug_map();

        for (k, v) in self.iter() {
            map.entry(k, v);
        }

        map.finish()
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub enum Value {
    Nil,
    Boolean(bool),
    Integer(i64),
    Number(f64),
    String(String),
    Table(Table),
}

impl Value {
    pub fn ty(&self) -> Type {
        match self {
            Value::Nil => Type::Nil,
            Value::Boolean(_) => Type::Boolean,
            Value::Integer(_) => Type::Integer,
            Value::Number(_) => Type::Number,
            Value::String(_) => Type::String,
            Value::Table(_) => Type::Table,
        }
    }
}

impl Default for Value {
    fn default() -> Self {
        Self::Nil
    }
}

impl From<bool> for Value {
    fn from(x: bool) -> Self {
        Self::Boolean(x)
    }
}

impl From<i64> for Value {
    fn from(x: i64) -> Self {
        Self::Integer(x)
    }
}

impl From<f64> for Value {
    fn from(x: f64) -> Self {
        Self::Number(x)
    }
}

impl From<String> for Value {
    fn from(x: String) -> Self {
        Self::String(x)
    }
}

impl From<Table> for Value {
    fn from(x: Table) -> Self {
        Self::Table(x)
    }
}

impl From<BTreeMap<Key, Value>> for Value {
    fn from(x: BTreeMap<Key, Value>) -> Self {
        Self::Table(x.into())
    }
}

impl Debug for Value {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Value::Nil => write!(f, "nil"),
            Value::Boolean(x) => x.fmt(f),
            Value::Integer(x) => x.fmt(f),
            Value::Number(x) => x.fmt(f),
            Value::String(x) => x.fmt(f),
            Value::Table(x) => x.fmt(f),
        }
    }
}

impl TryFrom<mlua::Value<'_>> for Value {
    type Error = Error;

    fn try_from(value: mlua::Value) -> Result<Self, Self::Error> {
        match value {
            mlua::Value::Nil => Ok(Value::Nil),
            mlua::Value::Boolean(x) => Ok(Value::Boolean(x)),
            mlua::Value::Integer(x) => Ok(Value::Integer(x)),
            mlua::Value::Number(x) => Ok(Value::Number(x)),
            mlua::Value::String(x) => Ok(Value::String(x.to_str()?.to_owned())),
            mlua::Value::Table(x) => Ok(Value::Table(x.try_into()?)),
            x => Err(Error::inconvertible(x)),
        }
    }
}
