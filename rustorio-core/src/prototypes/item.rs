use serde::{
    Deserialize,
    Serialize,
};

use super::{
    recipe::RecipePrototype,
    Id,
    Inherits,
    PrototypeBase,
};
use crate::types::{
    BeaconVisualizationTints,
    Color,
    Energy,
    EntityPrototype,
    EquipmentPrototype,
    FuelCategory,
    IconSpecification,
    ItemCountType,
    ItemPrototypeFlags,
    PlaceAsTile,
    SpriteVariations,
};

#[derive(Clone, Debug, Serialize, Deserialize, rustorio_data_derive::FromLuaTable)]
pub struct ItemPrototype {
    #[lua(flatten)]
    #[serde(flatten)]
    pub parent: PrototypeBase,

    pub stack_size: ItemCountType,

    #[lua(flatten)]
    pub icon_spec: IconSpecification,

    #[lua(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub place_result: Option<Id<EntityPrototype>>,

    #[lua(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placed_as_equipment_result: Option<Id<EquipmentPrototype>>,

    #[lua(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fuel_category: Option<Id<FuelCategory>>,

    #[lua(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub burnt_result: Option<Id<ItemPrototype>>,

    #[lua(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub place_as_tile: Option<PlaceAsTile>,

    #[lua(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pictures: Option<SpriteVariations>,

    #[lua(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<ItemPrototypeFlags>,

    #[lua(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_request_amount: Option<ItemCountType>,

    #[lua(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wire_count: Option<ItemCountType>,

    #[lua(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fuel_value: Option<Energy>,

    #[lua(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fuel_acceleration_multiplier: Option<f64>,

    #[lua(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fuel_top_speed_multiplier: Option<f64>,

    #[lua(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fuel_emissions_multiplier: Option<f64>,

    #[lua(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fuel_glow_color: Option<Color>,
}

impl Inherits for ItemPrototype {
    type Parent = PrototypeBase;

    fn parent(&self) -> &PrototypeBase {
        &self.parent
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, rustorio_data_derive::FromLuaTable)]
pub struct ItemIngredientPrototype {
    pub name: Id<ItemPrototype>,
    pub amount: u16,
    #[lua(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalyst_amount: Option<u16>,
}

impl From<(Id<ItemPrototype>, u16)> for ItemIngredientPrototype {
    fn from((name, amount): (Id<ItemPrototype>, u16)) -> Self {
        Self {
            name,
            amount,
            catalyst_amount: None,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, rustorio_data_derive::FromLuaTable)]
pub struct ItemProductPrototype {
    pub name: Id<ItemPrototype>,

    #[lua(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<u16>,

    #[lua(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_min: Option<u16>,

    #[lua(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_max: Option<u16>,

    #[lua(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub probability: Option<f64>,

    #[lua(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalyst_amount: Option<u16>,

    #[lua(default_with = "true")]
    pub show_details_in_recipe_tooltip: bool,
}

impl From<(Id<ItemPrototype>, u16)> for ItemProductPrototype {
    fn from((name, amount): (Id<ItemPrototype>, u16)) -> Self {
        Self {
            name,
            amount: Some(amount),
            amount_min: None,
            amount_max: None,
            probability: None,
            catalyst_amount: None,
            show_details_in_recipe_tooltip: true,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, rustorio_data_derive::FromLuaTable)]
pub struct ToolPrototype {
    #[lua(flatten)]
    #[serde(flatten)]
    pub parent: ItemPrototype,

    #[lua(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub durability: Option<f64>,

    #[lua(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub durability_description_key: Option<String>,

    #[lua(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub durability_description_value: Option<String>,

    #[lua(default)]
    pub infinite: bool,
}

impl Inherits for ToolPrototype {
    type Parent = ItemPrototype;

    fn parent(&self) -> &Self::Parent {
        &self.parent
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, rustorio_data_derive::FromLuaTable)]
pub struct ModulePrototype {
    #[lua(flatten)]
    #[serde(flatten)]
    pub parent: ItemPrototype,

    pub category: Id<ModuleCategory>,

    pub tier: u32,

    pub effect: Effect,

    #[lua(default_with = "true")]
    pub requires_beacon_alt_mode: bool,

    #[lua(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub limitation: Vec<Id<RecipePrototype>>,

    #[lua(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub limitation_blacklist: Vec<Id<RecipePrototype>>,

    #[lua(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limitation_message_key: Option<String>,

    #[lua(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub art_style: Option<String>,

    #[lua(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub beacon_tint: Option<BeaconVisualizationTints>,
}

impl Inherits for ModulePrototype {
    type Parent = ItemPrototype;

    fn parent(&self) -> &Self::Parent {
        &self.parent
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, rustorio_data_derive::FromLuaTable)]
pub struct ModuleCategory {
    #[lua(flatten)]
    #[serde(flatten)]
    parent: PrototypeBase,
}

impl Inherits for ModuleCategory {
    type Parent = PrototypeBase;

    fn parent(&self) -> &Self::Parent {
        &self.parent
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, rustorio_data_derive::FromLuaTable)]
pub struct Effect {
    #[lua(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumption: Option<EffectValue>,

    #[lua(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speed: Option<EffectValue>,

    #[lua(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub productivity: Option<EffectValue>,

    #[lua(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pollution: Option<EffectValue>,
}

#[derive(Clone, Debug, Serialize, Deserialize, rustorio_data_derive::FromLuaTable)]
pub struct EffectTypeLimitation();

#[derive(Clone, Debug, Serialize, Deserialize, rustorio_data_derive::FromLuaTable)]
pub struct EffectValue {
    pub bonus: f64,
}
