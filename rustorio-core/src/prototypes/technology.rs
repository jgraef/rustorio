use mlua::{
    Table,
    Value,
};
use rustorio_data::{
    FromLuaTable,
    FromLuaValue,
};
use rustorio_data_derive::FromLuaTable;
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

#[derive(Clone, Debug, Serialize, Deserialize, FromLuaTable)]
pub struct TechnologyPrototype {
    #[lua(flatten)]
    #[serde(flatten)]
    pub parent: PrototypeBase,

    /// icons, icon, icon_size (IconSpecification) :: IconSpecification
    #[lua(flatten)]
    pub icon_spec: IconSpecification,

    #[lua(flatten)]
    #[serde(flatten)]
    pub data: DifficultyDependentData<TechnologyData>,
}

impl Inherits for TechnologyPrototype {
    type Parent = PrototypeBase;

    fn parent(&self) -> &PrototypeBase {
        &self.parent
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, rustorio_data_derive::FromLuaTable)]
pub struct TechnologyData {
    #[lua(default)]
    pub upgrade: Option<bool>,

    #[lua(default)]
    pub enabled: bool,

    #[lua(default)]
    pub hidden: bool,

    #[lua(default)]
    pub visible_when_disabled: bool,

    #[lua(default)]
    pub ignore_tech_cost_multiplier: bool,

    #[lua(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<TechnologyUnit>,

    #[lua(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_level: Option<MaxLevel>,

    #[lua(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub effects: Vec<Modifier>,

    #[lua(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub prerequisites: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, rustorio_data_derive::FromLuaTable)]
pub struct TechnologyUnit {
    #[lua(flatten)]
    pub count: TechnologyUnitCount,
    pub time: f64,
    pub ingredients: Vec<IngredientPrototype>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
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

impl FromLuaTable for TechnologyUnitCount {
    fn from_lua_table(table: Table) -> Result<Self, rustorio_data::Error> {
        if let Some(formula) = table.get("count_formula").ok() {
            Ok(Self::Formula(formula))
        }
        else {
            let count = table.get("count")?;
            Ok(Self::Count(count))
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MaxLevel {
    Finite(u32),
    Infinite,
}

impl FromLuaValue for MaxLevel {
    fn from_lua_value(value: Value) -> Result<Self, rustorio_data::Error> {
        match value {
            Value::Integer(x) => Ok(Self::Finite(x as u32)),
            Value::String(s) if s == "infinite" => Ok(Self::Infinite),
            _ => Err(rustorio_data::Error::unexpected(value)),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, rustorio_data_derive::FromLuaTable)]
pub struct BaseModifier {
    #[lua(flatten)]
    pub icons: IconSpecification,
}

impl InheritsBase for BaseModifier {
    type Base = Self;

    fn base(&self) -> &Self::Base {
        self
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Modifier {
    //InserterStackSizeBonus(InserterStackSizeBonusModifier),
    //StackInserterCapacityBonus(StackInserterCapacityBonus),
    LaboratorySpeed(LaboratorySpeedModifier),
    //LaboratoryProductivity(LaboratoryProductivityModifier),
    //MaximumFollowingRobotsCount(MaximumFollowingRobotsCountModifier),
    // todo: implement all variants
    Todo(String),
}

impl FromLuaTable for Modifier {
    fn from_lua_table(table: Table) -> Result<Self, rustorio_data::Error> {
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

#[derive(Clone, Debug, Serialize, Deserialize, rustorio_data_derive::FromLuaTable)]
pub struct InserterStackSizeBonusModifier {
    // todo
}

#[derive(Clone, Debug, Serialize, Deserialize, rustorio_data_derive::FromLuaTable)]
pub struct SimpleModifier {
    #[lua(flatten)]
    pub parent: BaseModifier,
    pub modifier: f64,
}

impl Inherits for SimpleModifier {
    type Parent = BaseModifier;

    fn parent(&self) -> &Self::Parent {
        &self.parent
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, rustorio_data_derive::FromLuaTable)]
pub struct LaboratorySpeedModifier {
    #[lua(flatten)]
    pub parent: SimpleModifier,
}

impl Inherits for LaboratorySpeedModifier {
    type Parent = SimpleModifier;

    fn parent(&self) -> &Self::Parent {
        &self.parent
    }
}
