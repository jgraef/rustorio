use serde::{Deserialize, Serialize};

use crate::prototypes::{Prototype, Visitor};
use crate::types::*; // TODO: Import only specific types

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SolarPanel {
    /// energy_source :: EnergySource
    energy_source: EnergySource,

    /// picture :: SpriteVariations
    picture: SpriteVariations,

    /// production :: Energy
    production: Energy,

    /// overlay :: SpriteVariations (optional)
    overlay: Option<SpriteVariations>,
}

impl Prototype for SolarPanel {
    const TYPE: Option<&'static str> = Some("solar-panel");
}
