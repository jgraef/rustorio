use serde::{Serialize, Deserialize};
        
use super::{Prototype, type_stubs::*};
        
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FluidWagon {
    /// capacity :: double
    capacity: f64,    

    /// tank_count :: uint8 (optional)
    tank_count: Option<u8>,    

}

impl Prototype for FluidWagon {
    const TYPE: Option<&'static str> = Some("fluid-wagon");
}


