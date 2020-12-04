use serde::{Deserialize, Serialize};

use crate::prototypes::{Prototype, Visitor};
use crate::types::*; // TODO: Import only specific types

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Car {
    /// animation :: RotatedAnimation
    animation: RotatedAnimation,

    /// burner or energy_source :: EnergySource
    burner: Option<EnergySource>, // TODO enum
    energy_source: Option<EnergySource>,

    /// consumption :: Energy
    consumption: Energy,

    /// effectivity :: double
    effectivity: f64,

    /// inventory_size :: ItemStackIndex
    inventory_size: ItemStackIndex,

    /// rotation_speed :: double
    rotation_speed: f64,

    /// guns :: table of strings of prototype names (optional)
    guns: Option<Vec<String>>,

    /// has_belt_immunity :: bool (optional)
    has_belt_immunity: Option<bool>,

    /// immune_to_rock_impacts :: bool (optional)
    immune_to_rock_impacts: Option<bool>,

    /// immune_to_tree_impacts :: bool (optional)
    immune_to_tree_impacts: Option<bool>,

    /// light :: LightDefinition (optional)
    light: Option<LightDefinition>,

    /// render_layer :: RenderLayer (optional)
    render_layer: Option<RenderLayer>,

    /// sound_no_fuel :: Sound (optional)
    sound_no_fuel: Option<Sound>,

    /// tank_driving :: bool (optional)
    tank_driving: Option<bool>,

    /// track_particle_triggers :: FootstepTriggerEffectList (optional)
    track_particle_triggers: Option<FootstepTriggerEffectList>,

    /// turret_animation :: RotatedAnimation (optional)
    turret_animation: Option<RotatedAnimation>,

    /// turret_return_timeout :: uint32 (optional)
    turret_return_timeout: Option<u32>,

    /// turret_rotation_speed :: double (optional)
    turret_rotation_speed: Option<f64>,
}

impl Prototype for Car {
    const TYPE: Option<&'static str> = Some("car");
}
