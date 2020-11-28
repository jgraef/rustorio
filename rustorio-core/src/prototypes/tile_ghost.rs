use serde::{Serialize, Deserialize};
        
use super::{Prototype, type_stubs::*};
        
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TileGhost {

}

impl Prototype for TileGhost {
    const TYPE: Option<&'static str> = Some("tile-ghost");
}


