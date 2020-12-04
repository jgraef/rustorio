use serde::{Deserialize, Serialize};

use crate::prototypes::{Prototype, Visitor};
use crate::types::*; // TODO: Import only specific types

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Unit {
    /// attack_parameters :: AttackParameters
    attack_parameters: AttackParameters,

    /// distance_per_frame :: float
    distance_per_frame: f32,

    /// distraction_cooldown :: uint32
    distraction_cooldown: u32,

    /// movement_speed :: float
    movement_speed: f32,

    /// pollution_to_join_attack :: float
    pollution_to_join_attack: f32,

    /// run_animation :: RotatedAnimation
    run_animation: RotatedAnimation,

    /// vision_distance :: double
    vision_distance: f64,

    /// affected_by_tiles :: bool (optional)
    affected_by_tiles: Option<bool>,

    /// ai_settings :: UnitAISettings (optional)
    ai_settings: Option<UnitAISettings>,

    /// alternative_attacking_frame_sequence :: table (optional)
    alternative_attacking_frame_sequence: Option<Vec<Todo>>,

    /// can_open_gates :: bool (optional)
    can_open_gates: Option<bool>,

    /// dying_sound :: Sound (optional)
    dying_sound: Option<Sound>,

    /// has_belt_immunity :: bool (optional)
    has_belt_immunity: Option<bool>,

    /// light :: LightDefinition (optional)
    light: Option<LightDefinition>,

    /// max_pursue_distance :: double (optional)
    max_pursue_distance: Option<f64>,

    /// min_pursue_time :: uint32 (optional)
    min_pursue_time: Option<u32>,

    /// move_while_shooting :: bool (optional)
    move_while_shooting: Option<bool>,

    /// radar_range :: uint32 (optional)
    radar_range: Option<u32>,

    /// render_layer :: RenderLayer (optional)
    render_layer: Option<RenderLayer>,

    /// rotation_speed :: float (optional)
    rotation_speed: Option<f32>,

    /// running_sound_animation_positions :: table (array) of float (optional)
    running_sound_animation_positions: Option<Vec<f32>>,

    /// spawning_time_modifier :: double (optional)
    spawning_time_modifier: Option<f64>,

    /// walking_sound :: Sound (optional)
    walking_sound: Option<Sound>,
}

impl Prototype for Unit {
    const TYPE: Option<&'static str> = Some("unit");
}
