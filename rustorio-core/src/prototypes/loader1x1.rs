use serde::{Serialize, Deserialize};
        
use super::{Prototype, type_stubs::*};
        
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Loader1x1 {
    /// filter_count :: uint8
    filter_count: u8,    

    /// structure :: table
    structure: Vec<Todo>,

    /// belt_length :: double (optional)
    belt_length: Option<f64>,    

    /// container_distance :: double (optional)
    container_distance: Option<f64>,    

    /// structure_render_layer :: RenderLayer (optional)
    structure_render_layer: Option<RenderLayer>,    

}

impl Prototype for Loader1x1 {
    const TYPE: Option<&'static str> = Some("loader-1x1");
}


