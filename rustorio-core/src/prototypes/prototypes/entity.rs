use serde::{Deserialize, Serialize};

use crate::prototypes::{Prototype, Visitor};
use crate::types::*; // TODO: Import only specific types

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Entity {
    /// icons, icon,  icon_size (IconSpecification) :: IconSpecification
    icon_spec: IconSpecification,

    /// additional_pastable_entities :: table of string (optional)
    additional_pastable_entities: Option<Vec<String>>,

    /// alert_icon_scale :: float (optional)
    alert_icon_scale: Option<f32>,

    /// alert_icon_shift :: vector (optional)
    alert_icon_shift: Option<Vector2<f32>>,

    /// allow_copy_paste :: bool (optional)
    allow_copy_paste: Option<bool>,

    /// autoplace :: AutoplaceSpecification (optional)
    autoplace: Option<AutoplaceSpecification>,

    /// build_base_evolution_requirement :: double (optional)
    build_base_evolution_requirement: Option<f64>,

    /// build_sound :: Sound (optional)
    build_sound: Option<Sound>,

    /// close_sound :: Sound (optional)
    close_sound: Option<Sound>,

    /// collision_box :: BoundingBox (optional)
    collision_box: Option<BoundingBox>,

    /// collision_mask :: CollisionMask (optional)
    collision_mask: Option<CollisionMask>,

    /// created_effect :: Trigger (optional)
    created_effect: Option<Trigger>,

    /// created_smoke :: CreateTrivialSmokeEffectItem (optional)
    created_smoke: Option<CreateTrivialSmokeEffectItem>,

    /// drawing_box :: BoundingBox (optional)
    drawing_box: Option<BoundingBox>,

    /// emissions_per_second :: double (optional)
    emissions_per_second: Option<f64>,

    /// enemy_map_color :: Color (optional)
    enemy_map_color: Option<Color>,

    /// fast_replaceable_group :: string (optional)
    fast_replaceable_group: Option<String>,

    /// flags :: EntityPrototypeFlags (optional)
    flags: Option<EntityPrototypeFlags>,

    /// friendly_map_color :: Color (optional)
    friendly_map_color: Option<Color>,

    /// hit_visualization_box :: BoundingBox (optional)
    hit_visualization_box: Option<BoundingBox>,

    /// map_color :: Color (optional)
    map_color: Option<Color>,

    /// map_generator_bounding_box :: BoundingBox (optional)
    map_generator_bounding_box: Option<BoundingBox>,

    /// minable :: MinableProperties (optional)
    minable: Option<MinableProperties>,

    /// mined_sound :: Sound (optional)
    mined_sound: Option<Sound>,

    /// mining_sound :: Sound (optional)
    mining_sound: Option<Sound>,

    /// next_upgrade :: string (optional)
    next_upgrade: Option<String>,

    /// open_sound :: Sound (optional)
    open_sound: Option<Sound>,

    /// placeable_by :: ItemToPlace or table of ItemToPlace (optional)
    placeable_by: Option<Todo>,

    /// radius_visualisation_specification :: RadiusVisualisationSpecification (optional)
    radius_visualisation_specification: Option<RadiusVisualisationSpecification>,

    /// remains_when_mined :: string or table of string (optional)
    remains_when_mined: Option<Todo>,

    /// remove_decoratives :: string (optional)
    remove_decoratives: Option<String>,

    /// rotated_sound :: Sound (optional)
    rotated_sound: Option<Sound>,

    /// selectable_in_game :: bool (optional)
    selectable_in_game: Option<bool>,

    /// selection_box :: BoundingBox (optional)
    selection_box: Option<BoundingBox>,

    /// selection_priority :: uint8 (optional)
    selection_priority: Option<u8>,

    /// shooting_cursor_size :: double (optional)
    shooting_cursor_size: Option<f64>,

    /// sticker_box :: BoundingBox (optional)
    sticker_box: Option<BoundingBox>,

    /// subgroup :: string (optional)
    subgroup: Option<String>,

    /// tile_height :: uint32 (optional)
    tile_height: Option<u32>,

    /// tile_width :: uint32 (optional)
    tile_width: Option<u32>,

    /// trigger_target_mask :: TriggerTargetMask (optional)
    trigger_target_mask: Option<TriggerTargetMask>,

    /// vehicle_impact_sound :: Sound (optional)
    vehicle_impact_sound: Option<Sound>,

    /// water_reflection :: WaterReflectionDefinition (optional)
    water_reflection: Option<WaterReflectionDefinition>,

    /// working_sound :: WorkingSound (optional)
    working_sound: Option<WorkingSound>,
}

impl Prototype for Entity {
    const TYPE: Option<&'static str> = Some("None");
}
