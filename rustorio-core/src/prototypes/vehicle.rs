use serde::{Deserialize, Serialize};

use super::Prototype;
use crate::types::*; // TODO: Import only specific types

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Vehicle {
    /// braking_power or braking_force :: Energy or double
    braking_power: Option<Energy>,
    braking_force: Option<f64>, // TODO enum

    /// energy_per_hit_point :: double
    energy_per_hit_point: f64,

    /// friction or friction_force :: double
    friction: Option<f64>,
    friction_force: Option<f64>, // TODO enum

    /// weight :: double
    weight: f64,

    /// crash_trigger :: TriggerEffect (optional)
    crash_trigger: Option<TriggerEffect>,

    /// equipment_grid :: string (optional)
    equipment_grid: Option<String>,

    /// minimap_representation :: Sprite (optional)
    minimap_representation: Option<Sprite>,

    /// selected_minimap_representation :: Sprite (optional)
    selected_minimap_representation: Option<Sprite>,

    /// sound_minimum_speed :: double (optional)
    sound_minimum_speed: Option<f64>,

    /// sound_scaling_ratio :: double (optional)
    sound_scaling_ratio: Option<f64>,

    /// stop_trigger :: TriggerEffect (optional)
    stop_trigger: Option<TriggerEffect>,

    /// stop_trigger_speed :: double (optional)
    stop_trigger_speed: Option<f64>,

    /// terrain_friction_modifier :: float (optional)
    terrain_friction_modifier: Option<f32>,
}

impl Prototype for Vehicle {
    const TYPE: Option<&'static str> = Some("None");
}
