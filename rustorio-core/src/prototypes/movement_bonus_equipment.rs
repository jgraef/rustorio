use serde::{Deserialize, Serialize};

use super::Prototype;
use crate::types::*; // TODO: Import only specific types

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MovementBonusEquipment {
    /// energy_consumption :: Energy
    energy_consumption: Energy,

    /// movement_bonus :: double
    movement_bonus: f64,
}

impl Prototype for MovementBonusEquipment {
    const TYPE: Option<&'static str> = Some("movement-bonus-equipment");
}
