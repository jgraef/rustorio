use serde::{Deserialize, Serialize};

use super::Prototype;
use crate::types::*; // TODO: Import only specific types

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FluidTurret {
    /// activation_buffer_ratio :: float
    activation_buffer_ratio: f32,

    /// fluid_box :: FluidBox
    fluid_box: FluidBox,

    /// fluid_buffer_input_flow :: float
    fluid_buffer_input_flow: f32,

    /// fluid_buffer_size :: float
    fluid_buffer_size: f32,

    /// attacking_muzzle_animation_shift :: AnimatedVector (optional)
    attacking_muzzle_animation_shift: Option<AnimatedVector>,

    /// ending_attack_muzzle_animation_shift :: AnimatedVector (optional)
    ending_attack_muzzle_animation_shift: Option<AnimatedVector>,

    /// enough_fuel_indicator_picture :: Sprite4Way (optional)
    enough_fuel_indicator_picture: Option<Sprite4Way>,

    /// folded_muzzle_animation_shift :: AnimatedVector (optional)
    folded_muzzle_animation_shift: Option<AnimatedVector>,

    /// folding_muzzle_animation_shift :: AnimatedVector (optional)
    folding_muzzle_animation_shift: Option<AnimatedVector>,

    /// indicator_light :: LightDefinition (optional)
    indicator_light: Option<LightDefinition>,

    /// muzzle_animation :: Animation (optional)
    muzzle_animation: Option<Animation>,

    /// muzzle_light :: LightDefinition (optional)
    muzzle_light: Option<LightDefinition>,

    /// not_enough_fuel_indicator_picture :: Sprite4Way (optional)
    not_enough_fuel_indicator_picture: Option<Sprite4Way>,

    /// out_of_ammo_alert_icon :: Sprite (optional)
    out_of_ammo_alert_icon: Option<Sprite>,

    /// prepared_muzzle_animation_shift :: AnimatedVector (optional)
    prepared_muzzle_animation_shift: Option<AnimatedVector>,

    /// preparing_muzzle_animation_shift :: AnimatedVector (optional)
    preparing_muzzle_animation_shift: Option<AnimatedVector>,

    /// starting_attack_muzzle_animation_shift :: AnimatedVector (optional)
    starting_attack_muzzle_animation_shift: Option<AnimatedVector>,
}

impl Prototype for FluidTurret {
    const TYPE: Option<&'static str> = Some("fluid-turret");
}
