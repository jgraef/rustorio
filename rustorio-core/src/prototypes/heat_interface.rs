use serde::{Serialize, Deserialize};
        
use super::{Prototype, type_stubs::*};
        
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HeatInterface {
    /// heat_buffer :: HeatBuffer
    heat_buffer: HeatBuffer,    

    /// gui_mode :: string (optional)
    gui_mode: Option<String>,    

    /// picture :: Sprite (optional)
    picture: Option<Sprite>,    

}

impl Prototype for HeatInterface {
    const TYPE: Option<&'static str> = Some("heat-interface");
}


