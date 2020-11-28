use serde::{Serialize, Deserialize};
        
use super::{Prototype, type_stubs::*};
        
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ArtilleryFlare {
    /// life_time :: uint16
    life_time: u16,    

    /// pictures :: AnimationVariations
    pictures: AnimationVariations,    

    /// render_layer :: RenderLayer
    render_layer: RenderLayer,    

    /// render_layer_when_on_ground :: RenderLayer
    render_layer_when_on_ground: RenderLayer,    

    /// creation_shift :: vector (optional)
    creation_shift: Option<Vector2<f32>>,    

    /// early_death_ticks :: uint32 (optional)
    early_death_ticks: Option<u32>,    

    /// ended_in_water_trigger_effect :: TriggerEffect (optional)
    ended_in_water_trigger_effect: Option<TriggerEffect>,    

    /// initial_frame_speed :: float (optional)
    initial_frame_speed: Option<f32>,    

    /// initial_height :: float (optional)
    initial_height: Option<f32>,    

    /// initial_speed :: vector (optional)
    initial_speed: Option<Vector2<f32>>,    

    /// initial_vertical_speed :: float (optional)
    initial_vertical_speed: Option<f32>,    

    /// movement_modifier :: double (optional)
    movement_modifier: Option<f64>,    

    /// movement_modifier_when_on_ground :: double (optional)
    movement_modifier_when_on_ground: Option<f64>,    

    /// regular_trigger_effect :: TriggerEffect (optional)
    regular_trigger_effect: Option<TriggerEffect>,    

    /// regular_trigger_effect_frequency :: uint32 (optional)
    regular_trigger_effect_frequency: Option<u32>,    

    /// shadows :: AnimationVariations (optional)
    shadows: Option<AnimationVariations>,    

    /// shot_category :: string (optional)
    shot_category: Option<String>,    

    /// shots_per_flare :: uint32 (optional)
    shots_per_flare: Option<u32>,    

}

impl Prototype for ArtilleryFlare {
    const TYPE: Option<&'static str> = Some("artillery-flare");
}


