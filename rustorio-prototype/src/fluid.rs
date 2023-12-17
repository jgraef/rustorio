#[cfg(feature = "lua-api")]
use rustorio_lua_api::FromLuaTable;
#[cfg(feature = "serde")]
use serde::{
    Deserialize,
    Serialize,
};

use super::Id;
use crate::types::MaterialAmountType;

#[derive(Clone, Debug)]
#[cfg_attr(feature = "lua-api", derive(FromLuaTable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct FluidPrototype {
    // todo
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "lua-api", derive(FromLuaTable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct FluidIngredientPrototype {
    pub name: Id<FluidPrototype>,
    pub amount: f64,
    #[cfg_attr(feature = "lua-api", lua(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub temperature: Option<f64>,
    #[cfg_attr(feature = "lua-api", lua(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub minimum_temperature: Option<f64>,
    #[cfg_attr(feature = "lua-api", lua(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub maximum_temperature: Option<f64>,
    #[cfg_attr(feature = "lua-api", lua(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub catalyst_amount: Option<f64>,
    #[cfg_attr(feature = "lua-api", lua(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub fluidbox_index: Option<u32>,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "lua-api", derive(FromLuaTable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct FluidProductPrototype {
    pub name: Id<FluidPrototype>,
    pub amount: f64,
    #[cfg_attr(feature = "lua-api", lua(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub amount_min: Option<MaterialAmountType>,
    #[cfg_attr(feature = "lua-api", lua(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub amount_max: Option<MaterialAmountType>,
    #[cfg_attr(feature = "lua-api", lua(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub probability: Option<f64>,
    #[cfg_attr(feature = "lua-api", lua(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub catalyst_amount: Option<f64>,
    #[cfg_attr(feature = "lua-api", lua(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub temperature: Option<f64>,
    #[cfg_attr(feature = "lua-api", lua(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub fluidbox_index: Option<u32>,
    #[cfg_attr(feature = "lua-api", lua(default_with = "true"))]
    pub show_details_in_recipe_tooltip: bool,
}
