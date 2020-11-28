use serde::{Serialize, Deserialize};
        
use super::{Prototype, type_stubs::*};
        
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Locomotive {
    /// burner or energy_source :: EnergySource
    burner: Option<EnergySource>,
    energy_source: Option<EnergySource>, // TODO enum

    /// max_power :: Energy
    max_power: Energy,    

    /// reversing_power_modifier :: double
    reversing_power_modifier: f64,    

    /// front_light :: LightDefinition (optional)
    front_light: Option<LightDefinition>,    

}

impl Prototype for Locomotive {
    const TYPE: Option<&'static str> = Some("locomotive");
}


