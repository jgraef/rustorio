#[cfg(feature = "lua-api")]
use rustorio_lua_api::FromLuaTable;
#[cfg(feature = "serde")]
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

#[derive(Clone, Debug)]
#[cfg_attr(feature = "lua-api", derive(FromLuaTable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ItemPrototype {
    #[cfg_attr(feature = "lua-api", lua(flatten))]
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub parent: PrototypeBase,

    pub stack_size: ItemCountType,

    #[cfg_attr(feature = "lua-api", lua(flatten))]
    pub icon_spec: IconSpecification,

    #[cfg_attr(feature = "lua-api", lua(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub place_result: Option<Id<EntityPrototype>>,

    #[cfg_attr(feature = "lua-api", lua(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub placed_as_equipment_result: Option<Id<EquipmentPrototype>>,

    #[cfg_attr(feature = "lua-api", lua(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub fuel_category: Option<Id<FuelCategory>>,

    #[cfg_attr(feature = "lua-api", lua(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub burnt_result: Option<Id<ItemPrototype>>,

    #[cfg_attr(feature = "lua-api", lua(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub place_as_tile: Option<PlaceAsTile>,

    #[cfg_attr(feature = "lua-api", lua(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub pictures: Option<SpriteVariations>,

    #[cfg_attr(feature = "lua-api", lua(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub flags: Option<ItemPrototypeFlags>,

    #[cfg_attr(feature = "lua-api", lua(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub default_request_amount: Option<ItemCountType>,

    #[cfg_attr(feature = "lua-api", lua(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub wire_count: Option<ItemCountType>,

    #[cfg_attr(feature = "lua-api", lua(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub fuel_value: Option<Energy>,

    #[cfg_attr(feature = "lua-api", lua(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub fuel_acceleration_multiplier: Option<f64>,

    #[cfg_attr(feature = "lua-api", lua(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub fuel_top_speed_multiplier: Option<f64>,

    #[cfg_attr(feature = "lua-api", lua(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub fuel_emissions_multiplier: Option<f64>,

    #[cfg_attr(feature = "lua-api", lua(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub fuel_glow_color: Option<Color>,
}

impl Inherits for ItemPrototype {
    type Parent = PrototypeBase;

    fn parent(&self) -> &PrototypeBase {
        &self.parent
    }
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "lua-api", derive(FromLuaTable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ItemIngredientPrototype {
    pub name: Id<ItemPrototype>,
    pub amount: u16,
    #[cfg_attr(feature = "lua-api", lua(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
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

#[derive(Clone, Debug)]
#[cfg_attr(feature = "lua-api", derive(FromLuaTable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ItemProductPrototype {
    pub name: Id<ItemPrototype>,

    #[cfg_attr(feature = "lua-api", lua(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub amount: Option<u16>,

    #[cfg_attr(feature = "lua-api", lua(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub amount_min: Option<u16>,

    #[cfg_attr(feature = "lua-api", lua(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub amount_max: Option<u16>,

    #[cfg_attr(feature = "lua-api", lua(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub probability: Option<f64>,

    #[cfg_attr(feature = "lua-api", lua(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub catalyst_amount: Option<u16>,

    #[cfg_attr(feature = "lua-api", lua(default_with = "true"))]
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

#[derive(Clone, Debug)]
#[cfg_attr(feature = "lua-api", derive(FromLuaTable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ToolPrototype {
    #[cfg_attr(feature = "lua-api", lua(flatten))]
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub parent: ItemPrototype,

    #[cfg_attr(feature = "lua-api", lua(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub durability: Option<f64>,

    #[cfg_attr(feature = "lua-api", lua(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub durability_description_key: Option<String>,

    #[cfg_attr(feature = "lua-api", lua(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub durability_description_value: Option<String>,

    #[cfg_attr(feature = "lua-api", lua(default))]
    pub infinite: bool,
}

impl Inherits for ToolPrototype {
    type Parent = ItemPrototype;

    fn parent(&self) -> &Self::Parent {
        &self.parent
    }
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "lua-api", derive(FromLuaTable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ModulePrototype {
    #[cfg_attr(feature = "lua-api", lua(flatten))]
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub parent: ItemPrototype,

    pub category: Id<ModuleCategory>,

    pub tier: u32,

    pub effect: Effect,

    #[cfg_attr(feature = "lua-api", lua(default_with = "true"))]
    pub requires_beacon_alt_mode: bool,

    #[cfg_attr(feature = "lua-api", lua(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub limitation: Vec<Id<RecipePrototype>>,

    #[cfg_attr(feature = "lua-api", lua(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub limitation_blacklist: Vec<Id<RecipePrototype>>,

    #[cfg_attr(feature = "lua-api", lua(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub limitation_message_key: Option<String>,

    #[cfg_attr(feature = "lua-api", lua(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub art_style: Option<String>,

    #[cfg_attr(feature = "lua-api", lua(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub beacon_tint: Option<BeaconVisualizationTints>,
}

impl Inherits for ModulePrototype {
    type Parent = ItemPrototype;

    fn parent(&self) -> &Self::Parent {
        &self.parent
    }
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "lua-api", derive(FromLuaTable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ModuleCategory {
    #[cfg_attr(feature = "lua-api", lua(flatten))]
    #[cfg_attr(feature = "serde", serde(flatten))]
    parent: PrototypeBase,
}

impl Inherits for ModuleCategory {
    type Parent = PrototypeBase;

    fn parent(&self) -> &Self::Parent {
        &self.parent
    }
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "lua-api", derive(FromLuaTable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Effect {
    #[cfg_attr(feature = "lua-api", lua(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub consumption: Option<EffectValue>,

    #[cfg_attr(feature = "lua-api", lua(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub speed: Option<EffectValue>,

    #[cfg_attr(feature = "lua-api", lua(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub productivity: Option<EffectValue>,

    #[cfg_attr(feature = "lua-api", lua(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub pollution: Option<EffectValue>,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "lua-api", derive(FromLuaTable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct EffectTypeLimitation();

#[derive(Clone, Debug)]
#[cfg_attr(feature = "lua-api", derive(FromLuaTable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct EffectValue {
    pub bonus: f64,
}
