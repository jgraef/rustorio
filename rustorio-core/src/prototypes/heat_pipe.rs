use serde::{Serialize, Deserialize};
        
use super::{Prototype, type_stubs::*};
        
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HeatPipe {
    /// connection_sprites :: ConnectableEntityGraphics
    connection_sprites: ConnectableEntityGraphics,    

    /// heat_buffer :: HeatBuffer
    heat_buffer: HeatBuffer,    

    /// heat_glow_sprites :: ConnectableEntityGraphics
    heat_glow_sprites: ConnectableEntityGraphics,    

}

impl Prototype for HeatPipe {
    const TYPE: Option<&'static str> = Some("heat-pipe");
}


