use serde::{Deserialize, Serialize};

use super::Prototype;
use crate::types::*; // TODO: Import only specific types

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EquipmentCategory {}

impl Prototype for EquipmentCategory {
    const TYPE: Option<&'static str> = Some("equipment-category");
}
