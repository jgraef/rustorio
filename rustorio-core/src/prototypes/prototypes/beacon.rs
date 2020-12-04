use serde::{Deserialize, Serialize};

use crate::prototypes::{Prototype, Visitor};
use crate::types::*; // TODO: Import only specific types

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Beacon {
    /// distribution_effectivity :: double
    distribution_effectivity: f64,

    /// energy_source :: EnergySource
    energy_source: EnergySource,

    /// energy_usage :: Energy
    energy_usage: Energy,

    /// module_specification :: ModuleSpecification
    module_specification: ModuleSpecification,

    /// supply_area_distance :: double
    supply_area_distance: f64,

    /// allowed_effects :: EffectTypeLimitation (optional)
    allowed_effects: Option<EffectTypeLimitation>,

    /// animation :: Animation (optional)
    animation: Option<Animation>,

    /// base_picture :: Sprite (optional)
    base_picture: Option<Sprite>,

    /// graphics_set :: BeaconGraphicsSet (optional)
    graphics_set: Option<BeaconGraphicsSet>,

    /// radius_visualisation_picture :: Sprite (optional)
    radius_visualisation_picture: Option<Sprite>,
}

impl Prototype for Beacon {
    const TYPE: Option<&'static str> = Some("beacon");
}
