use serde::{Serialize, Deserialize};
        
use super::{Prototype, type_stubs::*};
        
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Pipe {
    /// fluid_box :: FluidBox
    fluid_box: FluidBox,    

    /// horizontal_window_bounding_box :: BoundingBox
    horizontal_window_bounding_box: BoundingBox,    

    /// pictures :: table
    pictures: Vec<Todo>,

    /// vertical_window_bounding_box :: BoundingBox
    vertical_window_bounding_box: BoundingBox,    

}

impl Prototype for Pipe {
    const TYPE: Option<&'static str> = Some("pipe");
}


