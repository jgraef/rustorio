use serde::{Deserialize, Serialize};

use crate::prototypes::{Prototype, Visitor};
use crate::types::*; // TODO: Import only specific types

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ElectricTurret {
    /// energy_source :: EnergySource
    energy_source: EnergySource,
}

impl Prototype for ElectricTurret {
    const TYPE: Option<&'static str> = Some("electric-turret");
}
