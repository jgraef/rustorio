use serde::{Deserialize, Serialize};

use super::Prototype;
use crate::types::*; // TODO: Import only specific types

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SolarPanelEquipment {
    /// power :: Energy
    power: Energy,
}

impl Prototype for SolarPanelEquipment {
    const TYPE: Option<&'static str> = Some("solar-panel-equipment");
}
