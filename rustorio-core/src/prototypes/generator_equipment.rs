use serde::{Serialize, Deserialize};
        
use super::{Prototype, type_stubs::*};
        
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GeneratorEquipment {
    /// power :: Energy
    power: Energy,    

    /// burner :: EnergySource (optional)
    burner: Option<EnergySource>,    

}

impl Prototype for GeneratorEquipment {
    const TYPE: Option<&'static str> = Some("generator-equipment");
}


