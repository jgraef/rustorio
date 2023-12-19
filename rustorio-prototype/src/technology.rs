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

use super::{
    recipe::IngredientPrototype,
    Inherits,
    InheritsBase,
    PrototypeBase,
};
use crate::types::{
    DifficultyDependentData,
    IconSpecification,
};

#[derive(Clone, Debug)]
#[cfg_attr(feature = "lua-api", derive(FromLuaTable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TechnologyPrototype {
    #[cfg_attr(feature = "lua-api", lua(flatten))]
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub parent: PrototypeBase,

    /// icons, icon, icon_size (IconSpecification) :: IconSpecification
    #[cfg_attr(feature = "lua-api", lua(flatten))]
    pub icon_spec: IconSpecification,

    #[cfg_attr(feature = "lua-api", lua(flatten))]
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub data: DifficultyDependentData<TechnologyData>,
}

impl Inherits for TechnologyPrototype {
    type Parent = PrototypeBase;

    fn parent(&self) -> &PrototypeBase {
        &self.parent
    }
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "lua-api", derive(FromLuaTable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TechnologyData {
    #[cfg_attr(feature = "lua-api", lua(default))]
    pub upgrade: Option<bool>,

    #[cfg_attr(feature = "lua-api", lua(default))]
    pub enabled: bool,

    #[cfg_attr(feature = "lua-api", lua(default))]
    pub hidden: bool,

    #[cfg_attr(feature = "lua-api", lua(default))]
    pub visible_when_disabled: bool,

    #[cfg_attr(feature = "lua-api", lua(default))]
    pub ignore_tech_cost_multiplier: bool,

    #[cfg_attr(feature = "lua-api", lua(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub unit: Option<TechnologyUnit>,

    #[cfg_attr(feature = "lua-api", lua(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub max_level: Option<MaxLevel>,

    #[cfg_attr(feature = "lua-api", lua(default))]
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Vec::is_empty")
    )]
    pub effects: Vec<Modifier>,

    #[cfg_attr(feature = "lua-api", lua(default))]
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Vec::is_empty")
    )]
    pub prerequisites: Vec<String>,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "lua-api", derive(FromLuaTable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TechnologyUnit {
    #[cfg_attr(feature = "lua-api", lua(flatten))]
    pub count: TechnologyUnitCount,
    pub time: f64,
    pub ingredients: Vec<IngredientPrototype>,
}

#[derive(Clone, Debug)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "lowercase")
)]
pub enum TechnologyUnitCount {
    Count(u64),
    Formula(String),
}

impl TechnologyUnitCount {
    pub fn as_count(&self) -> Option<u64> {
        match self {
            Self::Count(x) => Some(*x),
            Self::Formula(_) => None,
        }
    }

    pub fn as_formula(&self) -> Option<&str> {
        match self {
            Self::Count(_) => None,
            Self::Formula(formula) => Some(formula.as_str()),
        }
    }

    pub fn for_level(&self, _level: usize) -> u64 {
        match self {
            Self::Count(count) => *count,
            Self::Formula(_formula) => {
                todo!();
            }
        }
    }
}

#[cfg(feature = "lua-api")]
impl FromLuaTable for TechnologyUnitCount {
    fn from_lua_table(table: Table) -> Result<Self, rustorio_lua_api::Error> {
        if let Some(formula) = table.get("count_formula").ok() {
            Ok(Self::Formula(formula))
        }
        else {
            let count = table.get("count")?;
            Ok(Self::Count(count))
        }
    }
}

#[derive(Clone, Debug)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "lowercase")
)]
pub enum MaxLevel {
    Finite(u32),
    Infinite,
}

#[cfg(feature = "lua-api")]
impl FromLuaValue for MaxLevel {
    fn from_lua_value(value: Value) -> Result<Self, rustorio_lua_api::Error> {
        match value {
            Value::Integer(x) => Ok(Self::Finite(x as u32)),
            Value::String(s) if s == "infinite" => Ok(Self::Infinite),
            _ => Err(rustorio_lua_api::Error::unexpected(value)),
        }
    }
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "lua-api", derive(FromLuaTable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BaseModifier {
    #[cfg_attr(feature = "lua-api", lua(flatten))]
    pub icons: IconSpecification,
}

impl InheritsBase for BaseModifier {
    type Base = Self;

    fn base(&self) -> &Self::Base {
        self
    }
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Modifier {
    //InserterStackSizeBonus(InserterStackSizeBonusModifier),
    //StackInserterCapacityBonus(StackInserterCapacityBonus),
    LaboratorySpeed(LaboratorySpeedModifier),
    //LaboratoryProductivity(LaboratoryProductivityModifier),
    //MaximumFollowingRobotsCount(MaximumFollowingRobotsCountModifier),
    // todo: implement all variants
    Todo(String),
}

#[cfg(feature = "lua-api")]
impl FromLuaTable for Modifier {
    fn from_lua_table(table: Table) -> Result<Self, rustorio_lua_api::Error> {
        let ty: String = table.get("type")?;
        match ty.as_str() {
            /*"inserter-stack-size-bonus" => {
                Ok(Self::InserterStackSizeBonus(FromLuaTable::from_lua_table(
                    table,
                )?))
            }*/
            //"stack-inserter-capacity-bonus" => Ok(Self::StackInserterCapacityBonus(FromLuaTable::from_lua_table(table)?)),
            "laboratory-speed" => Ok(Self::LaboratorySpeed(FromLuaTable::from_lua_table(table)?)),
            //"laboratory-productivity" => Ok(Self::LaboratoryProductivity(FromLuaTable::from_lua_table(table)?)),
            //"maximum-following-robots-count" => Ok(Self::MaximumFollowingRobotsCount(FromLuaTable::from_lua_table(table)?)),
            _ => Ok(Self::Todo(ty)),
        }
    }
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "lua-api", derive(FromLuaTable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct InserterStackSizeBonusModifier {
    // todo
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "lua-api", derive(FromLuaTable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SimpleModifier {
    #[cfg_attr(feature = "lua-api", lua(flatten))]
    pub parent: BaseModifier,
    pub modifier: f64,
}

impl Inherits for SimpleModifier {
    type Parent = BaseModifier;

    fn parent(&self) -> &Self::Parent {
        &self.parent
    }
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "lua-api", derive(FromLuaTable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct LaboratorySpeedModifier {
    #[cfg_attr(feature = "lua-api", lua(flatten))]
    pub parent: SimpleModifier,
}

impl Inherits for LaboratorySpeedModifier {
    type Parent = SimpleModifier;

    fn parent(&self) -> &Self::Parent {
        &self.parent
    }
}
