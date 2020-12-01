use serde::{Deserialize, Serialize};

use super::Prototype;
use crate::types::*; // TODO: Import only specific types

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Reactor {
    /// consumption :: Energy
    consumption: Energy,

    /// energy_source :: EnergySource
    energy_source: EnergySource,

    /// heat_buffer :: HeatBuffer
    heat_buffer: HeatBuffer,

    /// working_light_picture :: Sprite
    working_light_picture: Sprite,

    /// connection_patches_connected :: SpriteVariations (optional)
    connection_patches_connected: Option<SpriteVariations>,

    /// connection_patches_disconnected :: SpriteVariations (optional)
    connection_patches_disconnected: Option<SpriteVariations>,

    /// default_fuel_glow_color :: Color (optional)
    default_fuel_glow_color: Option<Color>,

    /// heat_connection_patches_connected :: SpriteVariations (optional)
    heat_connection_patches_connected: Option<SpriteVariations>,

    /// heat_connection_patches_disconnected :: SpriteVariations (optional)
    heat_connection_patches_disconnected: Option<SpriteVariations>,

    /// heat_lower_layer_picture :: Sprite (optional)
    heat_lower_layer_picture: Option<Sprite>,

    /// light :: LightDefinition (optional)
    light: Option<LightDefinition>,

    /// lower_layer_picture :: Sprite (optional)
    lower_layer_picture: Option<Sprite>,

    /// meltdown_action :: Trigger (optional)
    meltdown_action: Option<Trigger>,

    /// neighbour_bonus :: double (optional)
    neighbour_bonus: Option<f64>,

    /// neighbour_collision_increase :: double (optional)
    neighbour_collision_increase: Option<f64>,

    /// picture :: Sprite (optional)
    picture: Option<Sprite>,

    /// scale_energy_usage :: bool (optional)
    scale_energy_usage: Option<bool>,

    /// use_fuel_glow_color :: bool (optional)
    use_fuel_glow_color: Option<bool>,
}

impl Prototype for Reactor {
    const TYPE: Option<&'static str> = Some("reactor");
}
