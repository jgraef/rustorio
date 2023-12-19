#[cfg(feature = "lua-api")]
use rustorio_lua_api::FromLuaTable;
#[cfg(feature = "serde")]
use serde::{
    Deserialize,
    Serialize,
};

use crate::{
    item::ItemPrototype,
    types::{
        Animation,
        BoundingBox,
        CollisionMask,
        Energy,
        EnergySource,
        EntityPrototypeFlag,
        EntityPrototypeFlags,
        IconSpecification,
        MinableProperties,
        ModuleSpecification,
        TriggerTargetMask,
    },
    Id,
    Inherits,
    PrototypeBase,
};

#[derive(Clone, Debug)]
#[cfg_attr(feature = "lua-api", derive(FromLuaTable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct EntityPrototype {
    #[cfg_attr(feature = "lua-api", lua(flatten))]
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub parent: PrototypeBase,

    #[cfg_attr(feature = "lua-api", lua(flatten))]
    pub icon_spec: IconSpecification,

    #[cfg_attr(feature = "lua-api", lua(default))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub collision_box: BoundingBox,

    #[cfg_attr(feature = "lua-api", lua(default))]
    pub collision_mask: CollisionMask,

    #[cfg_attr(feature = "lua-api", lua(default))]
    pub map_generator_bounding_box: Option<BoundingBox>,

    #[cfg_attr(feature = "lua-api", lua(default))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub selection_box: BoundingBox,

    #[cfg_attr(feature = "lua-api", lua(default))]
    pub drawing_box: Option<BoundingBox>,

    #[cfg_attr(feature = "lua-api", lua(default))]
    pub sticker_box: Option<BoundingBox>,

    #[cfg_attr(feature = "lua-api", lua(default))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub hit_visualization_box: BoundingBox,

    #[cfg_attr(feature = "lua-api", lua(default))]
    pub trigger_target_mask: Option<TriggerTargetMask>,

    #[cfg_attr(feature = "lua-api", lua(default))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub flags: EntityPrototypeFlags,

    #[cfg_attr(feature = "lua-api", lua(default))]
    pub minable: Option<MinableProperties>,

    // todo
    //pub subgroup: Option<Id<ItemSubGroup>>,
    #[cfg_attr(feature = "lua-api", lua(default_with = "true"))]
    #[cfg_attr(feature = "serde", serde(default = "bool_true"))]
    pub allow_copy_paste: bool,

    #[cfg_attr(feature = "lua-api", lua(default_with = "true"))]
    #[cfg_attr(feature = "serde", serde(default = "bool_true"))]
    pub selectable_in_game: bool,

    #[cfg_attr(
        feature = "lua-api",
        lua(default_with = "default_selection_priority()")
    )]
    #[cfg_attr(feature = "serde", serde(default = "default_selection_priority"))]
    pub selection_priority: u8,
    // todo
}

fn bool_true() -> bool {
    true
}

fn default_selection_priority() -> u8 {
    50
}

impl Inherits for EntityPrototype {
    type Parent = PrototypeBase;

    fn parent(&self) -> &Self::Parent {
        &self.parent
    }
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "lua-api", derive(FromLuaTable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct EntityWithHealthPrototype {
    #[cfg_attr(feature = "lua-api", lua(flatten))]
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub parent: EntityPrototype,

    #[cfg_attr(feature = "lua-api", lua(default))]
    pub max_health: Option<f64>,

    #[cfg_attr(feature = "lua-api", lua(default))]
    pub healing_per_tick: Option<f64>,

    #[cfg_attr(feature = "lua-api", lua(default))]
    pub repair_spee_modifier: Option<f64>,
    // todo
}

impl Inherits for EntityWithHealthPrototype {
    type Parent = EntityPrototype;

    fn parent(&self) -> &Self::Parent {
        &self.parent
    }
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "lua-api", derive(FromLuaTable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct EntityWithOwnerPrototype {
    #[cfg_attr(feature = "lua-api", lua(flatten))]
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub parent: EntityWithHealthPrototype,

    #[cfg_attr(feature = "lua-api", lua(default))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub is_military_target: bool,

    #[cfg_attr(feature = "lua-api", lua(default))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub allow_run_time_change_of_is_military_target: bool,
}

impl Inherits for EntityWithOwnerPrototype {
    type Parent = EntityWithHealthPrototype;

    fn parent(&self) -> &Self::Parent {
        &self.parent
    }
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "lua-api", derive(FromLuaTable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct LabPrototype {
    #[cfg_attr(feature = "lua-api", lua(flatten))]
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub parent: EntityWithOwnerPrototype,

    pub energy_usage: Energy,

    pub energy_source: EnergySource,

    pub on_animation: Animation,

    pub off_animation: Animation,

    pub inputs: Vec<Id<ItemPrototype>>,

    #[cfg_attr(feature = "lua-api", lua(default))]
    pub researching_speed: Option<f64>,

    #[cfg_attr(feature = "lua-api", lua(default))]
    pub base_productivity: Option<f64>,

    #[cfg_attr(feature = "lua-api", lua(default))]
    pub module_specification: Option<ModuleSpecification>,
}

impl Inherits for LabPrototype {
    type Parent = EntityWithOwnerPrototype;

    fn parent(&self) -> &Self::Parent {
        &self.parent
    }
}
