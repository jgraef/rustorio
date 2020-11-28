use serde::{Serialize, Deserialize};
        
use super::{Prototype, type_stubs::*};
        
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Particle {
    /// life_time :: uint16
    life_time: u16,    

    /// pictures :: AnimationVariations
    pictures: AnimationVariations,    

    /// render_layer :: RenderLayer
    render_layer: RenderLayer,    

    /// render_layer_when_on_ground :: RenderLayer
    render_layer_when_on_ground: RenderLayer,    

    /// draw_shadow_when_on_ground :: bool (optional)
    draw_shadow_when_on_ground: Option<bool>,    

    /// ended_in_water_trigger_effect :: TriggerEffect (optional)
    ended_in_water_trigger_effect: Option<TriggerEffect>,    

    /// ended_on_ground_trigger_effect :: TriggerEffect (optional)
    ended_on_ground_trigger_effect: Option<TriggerEffect>,    

    /// fade_away_duration :: uint16 (optional)
    fade_away_duration: Option<u16>,    

    /// mining_particle_frame_speed :: float (optional)
    mining_particle_frame_speed: Option<f32>,    

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

    /// vertical_acceleration :: float (optional)
    vertical_acceleration: Option<f32>,    

}

impl Prototype for Particle {
    const TYPE: Option<&'static str> = Some("optimized-particle");
}


