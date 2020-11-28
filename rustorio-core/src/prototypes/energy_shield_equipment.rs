use serde::{Serialize, Deserialize};
        
use super::{Prototype, type_stubs::*};
        
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


