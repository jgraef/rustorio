use serde::{Deserialize, Serialize};

use crate::prototypes::{Prototype, Visitor};
use crate::types::*; // TODO: Import only specific types

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Boiler {
    /// burning_cooldown :: uint32
    burning_cooldown: u32,

    /// energy_consumption :: Energy
    energy_consumption: Energy,

    /// energy_source :: EnergySource
    energy_source: EnergySource,

    /// fire :: table
    fire: Vec<Todo>,

    /// fire_glow :: table
    fire_glow: Vec<Todo>,

    /// fluid_box :: FluidBox
    fluid_box: FluidBox,

    /// output_fluid_box :: FluidBox
    output_fluid_box: FluidBox,

    /// structure :: table
    structure: Vec<Todo>,

    /// target_temperature :: double
    target_temperature: f64,

    /// fire_flicker_enabled :: bool (optional)
    fire_flicker_enabled: Option<bool>,

    /// fire_glow_flicker_enabled :: bool (optional)
    fire_glow_flicker_enabled: Option<bool>,

    /// mode :: string (optional)
    mode: Option<String>,

    /// patch :: table (optional)
    patch: Option<Vec<Todo>>,
}

impl Prototype for Boiler {
    const TYPE: Option<&'static str> = Some("boiler");
}
