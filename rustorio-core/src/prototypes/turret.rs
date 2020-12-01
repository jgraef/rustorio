use serde::{Deserialize, Serialize};

use super::Prototype;
use crate::types::*; // TODO: Import only specific types

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Turret {
    /// attack_parameters :: AttackParameters
    attack_parameters: AttackParameters,

    /// call_for_help_radius :: double
    call_for_help_radius: f64,

    /// folded_animation :: RotatedAnimation4Way
    folded_animation: RotatedAnimation4Way,

    /// alert_when_attacking :: bool (optional)
    alert_when_attacking: Option<bool>,

    /// allow_turning_when_starting_attack :: bool (optional)
    allow_turning_when_starting_attack: Option<bool>,

    /// attack_from_start_frame :: bool (optional)
    attack_from_start_frame: Option<bool>,

    /// attack_target_mask :: TriggerTargetMask (optional)
    attack_target_mask: Option<TriggerTargetMask>,

    /// attacking_animation :: RotatedAnimation4Way (optional)
    attacking_animation: Option<RotatedAnimation4Way>,

    /// attacking_speed :: float (optional)
    attacking_speed: Option<f32>,

    /// base_picture :: Animation4Way (optional)
    base_picture: Option<Animation4Way>,

    /// base_picture_render_layer :: RenderLayer (optional)
    base_picture_render_layer: Option<RenderLayer>,

    /// base_picture_secondary_draw_order :: uint8 (optional)
    base_picture_secondary_draw_order: Option<u8>,

    /// corpse :: string (optional)
    corpse: Option<String>,

    /// dying_sound :: Sound (optional)
    dying_sound: Option<Sound>,

    /// ending_attack_animation :: RotatedAnimation4Way (optional)
    ending_attack_animation: Option<RotatedAnimation4Way>,

    /// ending_attack_speed :: float (optional)
    ending_attack_speed: Option<f32>,

    /// energy_glow_animation :: RotatedAnimation4Way (optional)
    energy_glow_animation: Option<RotatedAnimation4Way>,

    /// folded_speed :: float (optional)
    folded_speed: Option<f32>,

    /// folded_speed_secondary :: float (optional)
    folded_speed_secondary: Option<f32>,

    /// folding_animation :: RotatedAnimation4Way (optional)
    folding_animation: Option<RotatedAnimation4Way>,

    /// folding_sound :: Sound (optional)
    folding_sound: Option<Sound>,

    /// folding_speed :: float (optional)
    folding_speed: Option<f32>,

    /// glow_light_intensity :: float (optional)
    glow_light_intensity: Option<f32>,

    /// gun_animation_render_layer :: RenderLayer (optional)
    gun_animation_render_layer: Option<RenderLayer>,

    /// gun_animation_secondary_draw_order :: uint8 (optional)
    gun_animation_secondary_draw_order: Option<u8>,

    /// ignore_target_mask :: TriggerTargetMask (optional)
    ignore_target_mask: Option<TriggerTargetMask>,

    /// integration :: Sprite (optional)
    integration: Option<Sprite>,

    /// prepare_range :: double (optional)
    prepare_range: Option<f64>,

    /// prepared_alternative_animation :: RotatedAnimation4Way (optional)
    prepared_alternative_animation: Option<RotatedAnimation4Way>,

    /// prepared_alternative_chance :: float (optional)
    prepared_alternative_chance: Option<f32>,

    /// prepared_alternative_sound :: Sound (optional)
    prepared_alternative_sound: Option<Sound>,

    /// prepared_alternative_speed :: float (optional)
    prepared_alternative_speed: Option<f32>,

    /// prepared_alternative_speed_secondary :: float (optional)
    prepared_alternative_speed_secondary: Option<f32>,

    /// prepared_animation :: RotatedAnimation4Way (optional)
    prepared_animation: Option<RotatedAnimation4Way>,

    /// prepared_sound :: Sound (optional)
    prepared_sound: Option<Sound>,

    /// prepared_speed :: float (optional)
    prepared_speed: Option<f32>,

    /// prepared_speed_secondary :: float (optional)
    prepared_speed_secondary: Option<f32>,

    /// preparing_animation :: RotatedAnimation4Way (optional)
    preparing_animation: Option<RotatedAnimation4Way>,

    /// preparing_sound :: Sound (optional)
    preparing_sound: Option<Sound>,

    /// preparing_speed :: float (optional)
    preparing_speed: Option<f32>,

    /// random_animation_offset :: bool (optional)
    random_animation_offset: Option<bool>,

    /// rotation_speed :: float (optional)
    rotation_speed: Option<f32>,

    /// secondary_animation :: bool (optional)
    secondary_animation: Option<bool>,

    /// shoot_in_prepare_state :: bool (optional)
    shoot_in_prepare_state: Option<bool>,

    /// starting_attack_animation :: RotatedAnimation4Way (optional)
    starting_attack_animation: Option<RotatedAnimation4Way>,

    /// starting_attack_sound :: Sound (optional)
    starting_attack_sound: Option<Sound>,

    /// starting_attack_speed :: float (optional)
    starting_attack_speed: Option<f32>,

    /// turret_base_has_direction :: bool (optional)
    turret_base_has_direction: Option<bool>,
}

impl Prototype for Turret {
    const TYPE: Option<&'static str> = Some("turret");
}
