use serde::{Serialize, Deserialize};
        
use super::{Prototype, type_stubs::*};
        
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SolarPanelEquipment {
    /// power :: Energy
    power: Energy,    

}

impl Prototype for SolarPanelEquipment {
    const TYPE: Option<&'static str> = Some("solar-panel-equipment");
}


