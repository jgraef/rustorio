use serde::{Deserialize, Serialize};

use super::Prototype;
use crate::types::*; // TODO: Import only specific types

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BeltImmunityEquipment {
    /// energy_consumption :: Energy
    energy_consumption: Energy,
}

impl Prototype for BeltImmunityEquipment {
    const TYPE: Option<&'static str> = Some("belt-immunity-equipment");
}
