use serde::{Serialize, Deserialize};
        
use super::{Prototype, type_stubs::*};
        
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ElectricTurret {
    /// energy_source :: EnergySource
    energy_source: EnergySource,    

}

impl Prototype for ElectricTurret {
    const TYPE: Option<&'static str> = Some("electric-turret");
}


