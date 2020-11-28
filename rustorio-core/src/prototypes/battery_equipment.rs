use serde::{Serialize, Deserialize};
        
use super::{Prototype, type_stubs::*};
        
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BatteryEquipment {

}

impl Prototype for BatteryEquipment {
    const TYPE: Option<&'static str> = Some("battery-equipment");
}


