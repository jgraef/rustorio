use serde::{Serialize, Deserialize};
        
use super::{Prototype, type_stubs::*};
        
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Fluid {
    /// base_color :: Color
    base_color: Color,    

    /// default_temperature :: double
    default_temperature: f64,    

    /// flow_color :: Color
    flow_color: Color,    

    /// icons, icon, icon_size (IconSpecification) :: IconSpecification
    icon_spec: IconSpecification,    

    /// max_temperature :: double
    max_temperature: f64,    

    /// emissions_multiplier :: double (optional)
    emissions_multiplier: Option<f64>,    

    /// fuel_value :: Energy (optional)
    fuel_value: Option<Energy>,    

    /// gas_temperature :: double (optional)
    gas_temperature: Option<f64>,    

    /// heat_capacity :: Energy (optional)
    heat_capacity: Option<Energy>,    

    /// hidden :: bool (optional)
    hidden: Option<bool>,    

    /// subgroup :: string (optional)
    subgroup: Option<String>,    

}

impl Prototype for Fluid {
    const TYPE: Option<&'static str> = Some("fluid");
}


