use serde::{Deserialize, Serialize};

use crate::prototypes::{Prototype, Visitor};
use crate::types::*; // TODO: Import only specific types

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Generator {
    /// effectivity :: double
    effectivity: f64,

    /// energy_source :: EnergySource
    energy_source: EnergySource,

    /// fluid_box :: FluidBox
    fluid_box: FluidBox,

    /// fluid_usage_per_tick :: double
    fluid_usage_per_tick: f64,

    /// horizontal_animation :: Animation
    horizontal_animation: Animation,

    /// maximum_temperature :: double
    maximum_temperature: f64,

    /// vertical_animation :: Animation
    vertical_animation: Animation,

    /// burns_fluid :: bool (optional)
    burns_fluid: Option<bool>,

    /// max_power_output :: Energy (optional)
    max_power_output: Option<Energy>,

    /// min_perceived_performance :: double (optional)
    min_perceived_performance: Option<f64>,

    /// performance_to_sound_speedup :: double (optional)
    performance_to_sound_speedup: Option<f64>,

    /// scale_fluid_usage :: bool (optional)
    scale_fluid_usage: Option<bool>,

    /// smoke :: table of SmokeSource (optional)
    smoke: Option<Vec<SmokeSource>>,
}

impl Prototype for Generator {
    const TYPE: Option<&'static str> = Some("generator");
}
