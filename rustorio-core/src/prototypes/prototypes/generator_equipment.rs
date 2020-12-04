use serde::{Deserialize, Serialize};

use crate::prototypes::{Prototype, Visitor};
use crate::types::*; // TODO: Import only specific types

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GeneratorEquipment {
    /// power :: Energy
    power: Energy,

    /// burner :: EnergySource (optional)
    burner: Option<EnergySource>,
}

impl Prototype for GeneratorEquipment {
    const TYPE: Option<&'static str> = Some("generator-equipment");
}
