use serde::{Serialize, Deserialize};
        
use super::{Prototype, type_stubs::*};
        
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EquipmentCategory {

}

impl Prototype for EquipmentCategory {
    const TYPE: Option<&'static str> = Some("equipment-category");
}


