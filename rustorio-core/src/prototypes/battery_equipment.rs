use serde::{Deserialize, Serialize};

use super::Prototype;
use crate::types::*; // TODO: Import only specific types

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BatteryEquipment {}

impl Prototype for BatteryEquipment {
    const TYPE: Option<&'static str> = Some("battery-equipment");
}
