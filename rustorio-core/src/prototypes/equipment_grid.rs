use serde::{Deserialize, Serialize};

use super::Prototype;
use crate::types::*; // TODO: Import only specific types

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EquipmentGrid {
    /// equipment_categories :: table of string
    equipment_categories: Vec<String>,

    /// height :: uint32
    height: u32,

    /// width :: uint32
    width: u32,

    /// locked :: bool (optional)
    locked: Option<bool>,
}

impl Prototype for EquipmentGrid {
    const TYPE: Option<&'static str> = Some("equipment-grid");
}
