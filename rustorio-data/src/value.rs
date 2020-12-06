use std::{
    collections::btree_map::{BTreeMap, Iter},
    fmt::{self, Debug, Formatter},
    iter::FromIterator,
    hash::{Hash, Hasher},
};

use derive_more::{From, Into, AsRef, AsMut, IntoIterator};
#[cfg(feature="serde")]
use serde::{Deserialize, Serialize};

use super::{FromLuaValue, FromLuaTable, Error};


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

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
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

impl PartialEq<()> for Key {
    fn eq(&self, _: &()) -> bool {
        match self {
            Key::Nil => true,
            _ => false,
        }
    }
}

impl PartialEq<bool> for Key {
    fn eq(&self, other: &bool) -> bool {
        match self {
            Key::Boolean(x) => x == other,
            _ => false,
        }
    }
}

impl PartialEq<i64> for Key {
    fn eq(&self, other: &i64) -> bool {
        match self {
            Key::Integer(x) => x == other,
            _ => false,
        }
    }
}

impl PartialEq<String> for Key {
    fn eq(&self, other: &String) -> bool {
        match self {
            Key::String(x) => x == other,
            _ => false,
        }
    }
}

impl PartialEq<str> for Key {
    fn eq(&self, other: &str) -> bool {
        match self {
            Key::String(x) => x == other,
            _ => false,
        }
    }
}

impl Hash for Key {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Key::Nil => Hash::hash(&(), state),
            Key::Boolean(x) => Hash::hash(&x, state),
            Key::Integer(x) => Hash::hash(&x, state),
            Key::String(x) => Hash::hash(&x, state),
        }
    }
}

impl From<()> for Key {
    fn from(_: ()) -> Self {
        Self::Nil
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

impl FromLuaValue for Key {
    fn from_lua_value(value: mlua::Value) -> Result<Self, Error> {
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

#[derive(Clone, Default, AsRef, AsMut, Into, From, IntoIterator)]
#[cfg_attr(feature="serde", derive(Serialize, Deserialize), serde(transparent))]
pub struct Table(BTreeMap<Key, Value>);

impl Table {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn iter(&self) -> Iter<Key, Value> {
        self.0.iter()
    }

    pub fn get(&self, k: &Key) -> Option<&Value> {
        self.0.get(k)
    }

    pub fn get_mut(&mut self, k: &Key) -> Option<&mut Value> {
        self.0.get_mut(k)
    }

    pub fn insert(&mut self, k: Key, v: Value) -> Option<Value> {
        self.0.insert(k, v)
    }
}

impl FromIterator<(Key, Value)> for Table {
    fn from_iter<T: IntoIterator<Item = (Key, Value)>>(iter: T) -> Self {
        Table(BTreeMap::from_iter(iter))
    }
}

impl FromLuaTable for Table {
    fn from_lua_table(table: mlua::Table) -> Result<Self, Error> {
        let mut t = BTreeMap::new();

        for r in table.pairs::<mlua::Value, mlua::Value>() {
            let (k, v) = r?;

            t.insert(Key::from_lua_value(k)?, Value::from_lua_value(v)?);
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

#[derive(Clone)]
#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
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

impl FromLuaValue for Value {
    fn from_lua_value(value: mlua::Value) -> Result<Self, Error> {
        match value {
            mlua::Value::Nil => Ok(Value::Nil),
            mlua::Value::Boolean(x) => Ok(Value::Boolean(x)),
            mlua::Value::Integer(x) => Ok(Value::Integer(x)),
            mlua::Value::Number(x) => Ok(Value::Number(x)),
            mlua::Value::String(x) => Ok(Value::String(x.to_str()?.to_owned())),
            mlua::Value::Table(x) => Ok(Value::Table(Table::from_lua_table(x)?)),
            x => Err(Error::inconvertible(x)),
        }
    }
}
