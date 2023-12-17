use rustorio_data_derive::FromLuaTable;
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
    prototypes::{
        fluid::FluidIngredientPrototype,
        item::ItemIngredientPrototype,
    },
    types::{
        DifficultyDependentData,
        IconSpecification,
        ItemOrFluid,
        Todo,
    },
};

#[derive(Clone, Debug, Serialize, Deserialize, FromLuaTable)]
pub struct RecipePrototype {
    #[lua(flatten)]
    #[serde(flatten)]
    parent: PrototypeBase,

    #[lua(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    category: Option<Id<RecipeCategory>>,

    #[lua(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    subgroup: Option<Id<Todo>>,

    #[lua(flatten)]
    icon_spec: IconSpecification,

    #[lua(flatten)]
    #[serde(flatten)]
    data: DifficultyDependentData<RecipeData>,
}

impl Inherits for RecipePrototype {
    type Parent = PrototypeBase;

    fn parent(&self) -> &PrototypeBase {
        &self.parent
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, FromLuaTable)]
pub struct RecipeCategory {
    #[lua(flatten)]
    #[serde(flatten)]
    parent: PrototypeBase,
}

#[derive(Clone, Debug, Serialize, Deserialize, FromLuaTable)]
pub struct RecipeData {
    #[lua(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    ingredients: Vec<IngredientPrototype>,

    #[lua(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    results: Vec<ProductPrototype>,

    #[lua(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<Id<ItemPrototype>>,

    #[lua(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    result_count: Option<u16>,

    #[lua(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    main_product: Option<String>,

    #[lua(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    energy_required: Option<f64>,
    #[lua(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    emissions_multiplier: Option<f64>,

    #[lua(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    requester_paste_multiplier: Option<u32>,

    #[lua(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    overload_multiplier: Option<u32>,

    #[lua(default)]
    allow_inserter_overload: bool,

    #[lua(default)]
    enabled: bool,

    #[lua(default)]
    hidden: bool,

    #[lua(default)]
    hide_from_stats: bool,

    #[lua(default)]
    hide_from_player_crafting: bool,

    #[lua(default)]
    allow_decomposition: bool,

    #[lua(default)]
    allow_as_intermediate: bool,

    #[lua(default)]
    allow_intermediates: bool,

    #[lua(default)]
    always_show_made_in: bool,

    #[lua(default)]
    show_amount_in_title: bool,

    #[lua(default)]
    always_show_products: bool,

    #[lua(default)]
    unlock_results: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, FromLuaTable)]
pub struct IngredientPrototype {
    #[lua(flatten)]
    #[serde(flatten)]
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

#[derive(Clone, Debug, Serialize, Deserialize, FromLuaTable)]
pub struct ProductPrototype {
    #[lua(flatten)]
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
