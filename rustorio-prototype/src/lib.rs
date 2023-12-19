pub mod achievement;
pub mod entity;
pub mod fluid;
pub mod item;
pub mod material;
pub mod recipe;
pub mod technology;
pub mod types;

use std::{
    collections::HashMap,
    hash::Hash,
    marker::PhantomData,
    sync::Arc,
};

use entity::LabPrototype;
#[cfg(feature = "lua-api")]
use rustorio_lua_api::{
    mlua::{
        Table,
        Value,
    },
    FromLuaTable,
    FromLuaValue,
};
#[cfg(feature = "serde")]
use serde::{
    Deserialize,
    Serialize,
};

use self::{
    achievement::AchievementPrototype,
    item::{
        ItemPrototype,
        ModulePrototype,
        ToolPrototype,
    },
    recipe::RecipePrototype,
    technology::TechnologyPrototype,
};
use crate::types::{
    LocalisedString,
    Order,
};

pub trait Inherits {
    type Parent;
    fn parent(&self) -> &Self::Parent;
}

pub trait InheritsBase {
    type Base;

    fn base(&self) -> &Self::Base;
}

impl<T: Inherits> InheritsBase for T
where
    T::Parent: InheritsBase,
{
    type Base = <T::Parent as InheritsBase>::Base;

    fn base(&self) -> &Self::Base {
        self.parent().base()
    }
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "lua-api", derive(FromLuaTable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PrototypeBase {
    pub r#type: String,
    pub name: String,

    #[cfg_attr(feature = "lua-api", lua(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub order: Option<Order>,

    #[cfg_attr(feature = "lua-api", lua(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub localised_name: Option<LocalisedString>,

    #[cfg_attr(feature = "lua-api", lua(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub localised_description: Option<LocalisedString>,
}

impl InheritsBase for PrototypeBase {
    type Base = Self;

    fn base(&self) -> &Self::Base {
        self
    }
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "lua-api", derive(FromLuaTable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Prototypes {
    #[cfg_attr(feature = "serde", serde(default))]
    achievement: PrototypeMap<AchievementPrototype>,

    #[cfg_attr(feature = "serde", serde(default))]
    technology: PrototypeMap<TechnologyPrototype>,

    #[cfg_attr(feature = "serde", serde(default))]
    recipe: PrototypeMap<RecipePrototype>,

    #[cfg_attr(feature = "serde", serde(default))]
    item: PrototypeMap<ItemPrototype>,

    #[cfg_attr(feature = "serde", serde(default))]
    tool: PrototypeMap<ToolPrototype>,

    #[cfg_attr(feature = "serde", serde(default))]
    module: PrototypeMap<ModulePrototype>,

    #[cfg_attr(feature = "serde", serde(default))]
    lab: PrototypeMap<LabPrototype>,
}

pub trait HasPrototypes<P: 'static> {
    fn get(&self, id: &Id<P>) -> Option<&P>;

    fn try_get(&self, id: &Id<P>) -> Result<&P, PrototypeNotFound> {
        self.get(id)
            .ok_or_else(|| PrototypeNotFound { id: id.to_string() })
    }

    fn iter(&self) -> impl Iterator<Item = &P>;
}

macro_rules! return_some {
    ($opt:expr) => {
        if let Some(value) = $opt {
            return Some(value);
        }
    };
}

impl HasPrototypes<AchievementPrototype> for Prototypes {
    fn get(&self, id: &Id<AchievementPrototype>) -> Option<&AchievementPrototype> {
        self.achievement.get(id)
    }

    fn iter(&self) -> impl Iterator<Item = &AchievementPrototype> {
        self.achievement.iter()
    }
}

impl HasPrototypes<TechnologyPrototype> for Prototypes {
    fn get(&self, id: &Id<TechnologyPrototype>) -> Option<&TechnologyPrototype> {
        self.technology.get(id)
    }

    fn iter(&self) -> impl Iterator<Item = &TechnologyPrototype> {
        self.technology.iter()
    }
}

impl HasPrototypes<RecipePrototype> for Prototypes {
    fn get(&self, id: &Id<RecipePrototype>) -> Option<&RecipePrototype> {
        self.recipe.get(id)
    }

    fn iter(&self) -> impl Iterator<Item = &RecipePrototype> {
        self.recipe.iter()
    }
}

impl HasPrototypes<ItemPrototype> for Prototypes {
    fn get(&self, id: &Id<ItemPrototype>) -> Option<&ItemPrototype> {
        return_some!(self.item.get(id));
        return_some!(self.tool.get(&id.downcast()).map(|x| x.parent()));
        return_some!(self.module.get(&id.downcast()).map(|x| x.parent()));
        None
    }

    fn iter(&self) -> impl Iterator<Item = &ItemPrototype> {
        self.item
            .iter()
            .chain(self.tool.iter().map(|x| x.parent()))
            .chain(self.module.iter().map(|x| x.parent()))
    }
}

impl HasPrototypes<ToolPrototype> for Prototypes {
    fn get(&self, id: &Id<ToolPrototype>) -> Option<&ToolPrototype> {
        self.tool.get(id)
    }

    fn iter(&self) -> impl Iterator<Item = &ToolPrototype> {
        self.tool.iter()
    }
}

impl HasPrototypes<ModulePrototype> for Prototypes {
    fn get(&self, id: &Id<ModulePrototype>) -> Option<&ModulePrototype> {
        self.module.get(id)
    }

    fn iter(&self) -> impl Iterator<Item = &ModulePrototype> {
        self.module.iter()
    }
}

impl HasPrototypes<LabPrototype> for Prototypes {
    fn get(&self, id: &Id<LabPrototype>) -> Option<&LabPrototype> {
        self.lab.get(id)
    }

    fn iter(&self) -> impl Iterator<Item = &LabPrototype> {
        self.lab.iter()
    }
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize), serde(transparent))]
pub struct PrototypeMap<P>(HashMap<String, P>);

impl<P> Default for PrototypeMap<P> {
    fn default() -> Self {
        Self(Default::default())
    }
}

#[cfg(feature = "lua-api")]
impl<P: FromLuaTable> FromLuaTable for PrototypeMap<P> {
    fn from_lua_table(table: Table) -> Result<Self, rustorio_lua_api::Error> {
        let d: HashMap<String, P> = FromLuaTable::from_lua_table(table)?;
        Ok(Self(d))
    }
}

impl<P> PrototypeMap<P> {
    pub fn get(&self, id: &Id<P>) -> Option<&P> {
        self.0.get(id.as_str())
    }

    pub fn try_get(&self, id: &Id<P>) -> Result<&P, PrototypeNotFound> {
        self.get(id)
            .ok_or_else(|| PrototypeNotFound { id: id.to_string() })
    }

    pub fn iter(&self) -> impl Iterator<Item = &P> {
        self.0.values()
    }
}

#[derive(Debug, thiserror::Error)]
#[error("prototype not found: {id}")]
pub struct PrototypeNotFound {
    id: String,
}

#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(from = "String", into = "String")
)]
pub struct Id<P> {
    id: Arc<String>,
    _t: PhantomData<P>,
}

impl<P> Id<P> {
    pub fn upcast<U>(&self) -> Id<U>
    where
        P: Inherits<Parent = U>,
    {
        Id {
            id: self.id.clone(),
            _t: PhantomData,
        }
    }

    pub fn downcast<U>(&self) -> Id<U>
    where
        U: Inherits<Parent = P>,
    {
        Id {
            id: self.id.clone(),
            _t: PhantomData,
        }
    }

    pub fn as_str(&self) -> &str {
        &self.id
    }
}

impl<P> Clone for Id<P> {
    fn clone(&self) -> Self {
        Self {
            id: self.id.clone(),
            _t: PhantomData,
        }
    }
}

impl<P> From<String> for Id<P> {
    fn from(id: String) -> Self {
        Id {
            id: Arc::new(id),
            _t: PhantomData,
        }
    }
}

impl<P> From<Arc<String>> for Id<P> {
    fn from(id: Arc<String>) -> Self {
        Self {
            id,
            _t: PhantomData,
        }
    }
}

impl<'a, P> From<&'a str> for Id<P> {
    fn from(id: &'a str) -> Self {
        Self {
            id: Arc::new(id.to_owned()),
            _t: PhantomData,
        }
    }
}

impl<P> From<Id<P>> for String {
    fn from(value: Id<P>) -> Self {
        value.id.to_string()
    }
}

#[cfg(feature = "lua-api")]
impl<P> FromLuaValue for Id<P> {
    fn from_lua_value(value: Value) -> Result<Self, rustorio_lua_api::Error> {
        Ok(String::from_lua_value(value)?.into())
    }
}

impl<P> std::fmt::Display for Id<P> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id)
    }
}

impl<P> std::fmt::Debug for Id<P> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\"{}\"", self.id)
    }
}

impl<P> PartialEq for Id<P> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<P> Eq for Id<P> {}

impl<P> Hash for Id<P> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}
