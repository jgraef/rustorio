use serde::{Deserialize, Serialize};

use super::Prototype;
use crate::types::*; // TODO: Import only specific types

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CraftingMachine {
    /// crafting_categories :: table of strings
    crafting_categories: Vec<String>,

    /// crafting_speed :: double
    crafting_speed: f64,

    /// energy_source :: EnergySource
    energy_source: EnergySource,

    /// energy_usage :: Energy
    energy_usage: Energy,

    /// allowed_effects :: EffectTypeLimitation (optional)
    allowed_effects: Option<EffectTypeLimitation>,

    /// always_draw_idle_animation :: bool (optional)
    always_draw_idle_animation: Option<bool>,

    /// animation :: Animation4Way (optional)
    animation: Option<Animation4Way>,

    /// base_productivity :: float (optional)
    base_productivity: Option<f32>,

    /// default_recipe_tint :: table of Color (optional)
    default_recipe_tint: Option<Vec<Color>>,

    /// draw_entity_info_icon_background :: bool (optional)
    draw_entity_info_icon_background: Option<bool>,

    /// entity_info_icon_shift :: vector (optional)
    entity_info_icon_shift: Option<Vector2<f32>>,

    /// fluid_boxes :: table of FluidBox (optional)
    fluid_boxes: Option<Vec<FluidBox>>,

    /// idle_animation :: Animation4Way (optional)
    idle_animation: Option<Animation4Way>,

    /// match_animation_speed_to_activity :: bool (optional)
    match_animation_speed_to_activity: Option<bool>,

    /// module_specification :: ModuleSpecification (optional)
    module_specification: Option<ModuleSpecification>,

    /// return_ingredients_on_change :: bool (optional)
    return_ingredients_on_change: Option<bool>,

    /// scale_entity_info_icon :: bool (optional)
    scale_entity_info_icon: Option<bool>,

    /// shift_animation_transition_duration :: uint16 (optional)
    shift_animation_transition_duration: Option<u16>,

    /// shift_animation_waypoint_stop_duration :: uint16 (optional)
    shift_animation_waypoint_stop_duration: Option<u16>,

    /// shift_animation_waypoints :: table of table (array) of vector (optional)
    shift_animation_waypoints: Option<Vec<Vec<Vector2<f32>>>>,

    /// show_recipe_icon :: bool (optional)
    show_recipe_icon: Option<bool>,

    /// status_colors :: table of Color (optional)
    status_colors: Option<Vec<Color>>,

    /// working_visualisations :: table of WorkingVisualisation (optional)
    working_visualisations: Option<Vec<WorkingVisualisation>>,
}

impl Prototype for CraftingMachine {
    const TYPE: Option<&'static str> = Some("None");
}
