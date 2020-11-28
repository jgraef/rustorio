use serde::{Serialize, Deserialize};
        
use super::{Prototype, type_stubs::*};
        
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BeltImmunityEquipment {
    /// energy_consumption :: Energy
    energy_consumption: Energy,    

}

impl Prototype for BeltImmunityEquipment {
    const TYPE: Option<&'static str> = Some("belt-immunity-equipment");
}


