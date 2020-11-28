use serde::{Serialize, Deserialize};
        
use super::{Prototype, type_stubs::*};
        
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


