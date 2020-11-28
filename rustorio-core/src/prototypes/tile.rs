use serde::{Serialize, Deserialize};
        
use super::{Prototype, type_stubs::*};
        
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Tile {
    /// collision_mask :: CollisionMask
    collision_mask: CollisionMask,    

    /// layer :: uint8
    layer: u8,    

    /// map_color :: Color
    map_color: Color,    

    /// pollution_absorption_per_second :: double
    pollution_absorption_per_second: f64,    

    /// variants :: TileTransitions
    variants: TileTransitions,    

    /// allowed_neighbors :: table of string (optional)
    allowed_neighbors: Option<Vec<String>>,

    /// autoplace :: AutoplaceSpecification (optional)
    autoplace: Option<AutoplaceSpecification>,    

    /// build_sound :: table (optional)
    build_sound: Option<Vec<Todo>>,

    /// can_be_part_of_blueprint :: bool (optional)
    can_be_part_of_blueprint: Option<bool>,    

    /// decorative_removal_probability :: float (optional)
    decorative_removal_probability: Option<f32>,    

    /// draw_in_water_layer :: bool (optional)
    draw_in_water_layer: Option<bool>,    

    /// effect :: string (optional)
    effect: Option<String>,    

    /// effect_color :: Color (optional)
    effect_color: Option<Color>,    

    /// effect_is_opaque :: bool (optional)
    effect_is_opaque: Option<bool>,    

    /// layer_group :: string (optional)
    layer_group: Option<String>,    

    /// minable :: MinableProperties (optional)
    minable: Option<MinableProperties>,    

    /// mined_sound :: Sound (optional)
    mined_sound: Option<Sound>,    

    /// needs_correction :: bool (optional)
    needs_correction: Option<bool>,    

    /// next_direction :: string (optional)
    next_direction: Option<String>,    

    /// placeable_by :: ItemToPlace or table of ItemToPlace (optional)
    placeable_by: Option<Todo>,

    /// scorch_mark_color :: Color (optional)
    scorch_mark_color: Option<Color>,    

    /// tint :: Color (optional)
    tint: Option<Color>,    

    /// transition_merges_with_tile :: string (optional)
    transition_merges_with_tile: Option<String>,    

    /// transition_overlay_layer_offset :: uint8 (optional)
    transition_overlay_layer_offset: Option<u8>,    

    /// transitions :: table of TileTransitions (optional)
    transitions: Option<Vec<TileTransitions>>,    

    /// transitions_between_transitions :: table of TileTransitions (optional)
    transitions_between_transitions: Option<Vec<TileTransitions>>,    

    /// trigger_effect :: TriggerEffect (optional)
    trigger_effect: Option<TriggerEffect>,    

    /// vehicle_friction_modifier :: double (optional)
    vehicle_friction_modifier: Option<f64>,    

    /// walking_sound :: Sound (optional)
    walking_sound: Option<Sound>,    

    /// walking_speed_modifier :: double (optional)
    walking_speed_modifier: Option<f64>,    

}

impl Prototype for Tile {
    const TYPE: Option<&'static str> = Some("tile");
}


