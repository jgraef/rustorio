use serde::{Serialize, Deserialize};
        
use super::{Prototype, type_stubs::*};
        
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PipeToGround {
    /// fluid_box :: FluidBox
    fluid_box: FluidBox,    

    /// pictures :: table of Sprite
    pictures: Vec<Sprite>,    

}

impl Prototype for PipeToGround {
    const TYPE: Option<&'static str> = Some("pipe-to-ground");
}


