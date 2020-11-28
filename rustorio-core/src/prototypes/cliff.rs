use serde::{Serialize, Deserialize};
        
use super::{Prototype, type_stubs::*};
        
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Cliff {
    /// grid_offset :: vector
    grid_offset: Vector2<f32>,    

    /// grid_size :: vector
    grid_size: Vector2<f32>,    

    /// orientations :: table of OrientedCliffPrototype
    orientations: Vec<OrientedCliffPrototype>,    

    /// cliff_explosive :: string (optional)
    cliff_explosive: Option<String>,    

    /// cliff_height :: float (optional)
    cliff_height: Option<f32>,    

}

impl Prototype for Cliff {
    const TYPE: Option<&'static str> = Some("cliff");
}


