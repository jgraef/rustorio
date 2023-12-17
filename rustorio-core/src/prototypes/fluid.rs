use serde::{
    Deserialize,
    Serialize,
};

use super::Id;
use crate::types::MaterialAmountType;

#[derive(Clone, Debug, Serialize, Deserialize, rustorio_data_derive::FromLuaTable)]
pub struct FluidPrototype {
    // todo
}

#[derive(Clone, Debug, Serialize, Deserialize, rustorio_data_derive::FromLuaTable)]
pub struct FluidIngredientPrototype {
    pub name: Id<FluidPrototype>,
    pub amount: f64,
    #[lua(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f64>,
    #[lua(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_temperature: Option<f64>,
    #[lua(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_temperature: Option<f64>,
    #[lua(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalyst_amount: Option<f64>,
    #[lua(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fluidbox_index: Option<u32>,
}

#[derive(Clone, Debug, Serialize, Deserialize, rustorio_data_derive::FromLuaTable)]
pub struct FluidProductPrototype {
    pub name: Id<FluidPrototype>,
    pub amount: f64,
    #[lua(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_min: Option<MaterialAmountType>,
    #[lua(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_max: Option<MaterialAmountType>,
    #[lua(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub probability: Option<f64>,
    #[lua(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalyst_amount: Option<f64>,
    #[lua(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f64>,
    #[lua(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fluidbox_index: Option<u32>,
    #[lua(default_with = "true")]
    pub show_details_in_recipe_tooltip: bool,
}
