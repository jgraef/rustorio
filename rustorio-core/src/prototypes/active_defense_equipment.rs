use serde::{Deserialize, Serialize};

use super::Prototype;
use crate::types::*; // TODO: Import only specific types

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ActiveDefenseEquipment {
    /// attack_parameters :: AttackParameters
    attack_parameters: AttackParameters,

    /// automatic :: bool
    automatic: bool,
}

impl Prototype for ActiveDefenseEquipment {
    const TYPE: Option<&'static str> = Some("active-defense-equipment");
}
