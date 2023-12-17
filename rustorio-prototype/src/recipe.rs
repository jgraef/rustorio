#[cfg(feature = "lua-api")]
use rustorio_lua_api::FromLuaTable;
#[cfg(feature = "serde")]
use serde::{
    Deserialize,
    Serialize,
};

use super::{
    fluid::FluidProductPrototype,
    item::{
        ItemProductPrototype,
        ItemPrototype,
    },
    Id,
    Inherits,
    PrototypeBase,
};
use crate::{
    fluid::FluidIngredientPrototype,
    item::ItemIngredientPrototype,
    types::{
        DifficultyDependentData,
        IconSpecification,
        ItemOrFluid,
        Todo,
    },
};

#[derive(Clone, Debug)]
#[cfg_attr(feature = "lua-api", derive(FromLuaTable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RecipePrototype {
    #[cfg_attr(feature = "lua-api", lua(flatten))]
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub parent: PrototypeBase,

    #[cfg_attr(feature = "lua-api", lua(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub category: Option<Id<RecipeCategory>>,

    #[cfg_attr(feature = "lua-api", lua(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub subgroup: Option<Id<Todo>>,

    #[cfg_attr(feature = "lua-api", lua(flatten))]
    pub icon_spec: IconSpecification,

    #[cfg_attr(feature = "lua-api", lua(flatten))]
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub data: DifficultyDependentData<RecipeData>,
}

impl Inherits for RecipePrototype {
    type Parent = PrototypeBase;

    fn parent(&self) -> &PrototypeBase {
        &self.parent
    }
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "lua-api", derive(FromLuaTable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RecipeCategory {
    #[cfg_attr(feature = "lua-api", lua(flatten))]
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub parent: PrototypeBase,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "lua-api", derive(FromLuaTable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RecipeData {
    #[cfg_attr(feature = "lua-api", lua(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub ingredients: Vec<IngredientPrototype>,

    #[cfg_attr(feature = "lua-api", lua(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub results: Vec<ProductPrototype>,

    #[cfg_attr(feature = "lua-api", lua(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub result: Option<Id<ItemPrototype>>,

    #[cfg_attr(feature = "lua-api", lua(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub result_count: Option<u16>,

    #[cfg_attr(feature = "lua-api", lua(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub main_product: Option<String>,

    #[cfg_attr(feature = "lua-api", lua(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub energy_required: Option<f64>,

    #[cfg_attr(feature = "lua-api", lua(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub emissions_multiplier: Option<f64>,

    #[cfg_attr(feature = "lua-api", lua(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub requester_paste_multiplier: Option<u32>,

    #[cfg_attr(feature = "lua-api", lua(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub overload_multiplier: Option<u32>,

    #[cfg_attr(feature = "lua-api", lua(default))]
    pub allow_inserter_overload: bool,

    #[cfg_attr(feature = "lua-api", lua(default))]
    pub enabled: bool,

    #[cfg_attr(feature = "lua-api", lua(default))]
    pub hidden: bool,

    #[cfg_attr(feature = "lua-api", lua(default))]
    pub hide_from_stats: bool,

    #[cfg_attr(feature = "lua-api", lua(default))]
    pub hide_from_player_crafting: bool,

    #[cfg_attr(feature = "lua-api", lua(default))]
    pub allow_decomposition: bool,

    #[cfg_attr(feature = "lua-api", lua(default))]
    pub allow_as_intermediate: bool,

    #[cfg_attr(feature = "lua-api", lua(default))]
    pub allow_intermediates: bool,

    #[cfg_attr(feature = "lua-api", lua(default))]
    pub always_show_made_in: bool,

    #[cfg_attr(feature = "lua-api", lua(default))]
    pub show_amount_in_title: bool,

    #[cfg_attr(feature = "lua-api", lua(default))]
    pub always_show_products: bool,

    #[cfg_attr(feature = "lua-api", lua(default))]
    pub unlock_results: bool,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "lua-api", derive(FromLuaTable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct IngredientPrototype {
    #[cfg_attr(feature = "lua-api", lua(flatten))]
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub item_or_fluid: ItemOrFluid<ItemIngredientPrototype, FluidIngredientPrototype>,
}

impl IngredientPrototype {
    pub fn as_item(&self) -> Option<&ItemIngredientPrototype> {
        self.item_or_fluid.as_item()
    }

    pub fn as_fluid(&self) -> Option<&FluidIngredientPrototype> {
        self.item_or_fluid.as_fluid()
    }
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "lua-api", derive(FromLuaTable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ProductPrototype {
    #[cfg_attr(feature = "lua-api", lua(flatten))]
    pub item_or_fluid: ItemOrFluid<ItemProductPrototype, FluidProductPrototype>,
}

impl ProductPrototype {
    pub fn as_item(&self) -> Option<&ItemProductPrototype> {
        self.item_or_fluid.as_item()
    }

    pub fn as_fluid(&self) -> Option<&FluidProductPrototype> {
        self.item_or_fluid.as_fluid()
    }
}
