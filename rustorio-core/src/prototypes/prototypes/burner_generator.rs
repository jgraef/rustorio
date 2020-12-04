use serde::{Deserialize, Serialize};

use crate::prototypes::{Prototype, Visitor};
use crate::types::*; // TODO: Import only specific types

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BurnerGenerator {
    /// animation :: Animation4Way
    animation: Animation4Way,

    /// burner :: EnergySource
    burner: EnergySource,

    /// energy_source :: EnergySource
    energy_source: EnergySource,

    /// max_power_output :: Energy
    max_power_output: Energy,

    /// always_draw_idle_animation :: bool (optional)
    always_draw_idle_animation: Option<bool>,

    /// idle_animation :: Animation4Way (optional)
    idle_animation: Option<Animation4Way>,

    /// min_perceived_performance :: double (optional)
    min_perceived_performance: Option<f64>,

    /// performance_to_sound_speedup :: double (optional)
    performance_to_sound_speedup: Option<f64>,
}

impl Prototype for BurnerGenerator {
    const TYPE: Option<&'static str> = Some("burner-generator");
}
