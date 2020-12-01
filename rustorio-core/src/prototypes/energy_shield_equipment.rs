use serde::{Deserialize, Serialize};

use super::Prototype;
use crate::types::*; // TODO: Import only specific types

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EnergyShieldEquipment {
    /// energy_per_shield :: Energy
    energy_per_shield: Energy,

    /// max_shield_value :: float
    max_shield_value: f32,
}

impl Prototype for EnergyShieldEquipment {
    const TYPE: Option<&'static str> = Some("energy-shield-equipment");
}
