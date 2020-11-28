use serde::{Serialize, Deserialize};
        
use super::{Prototype, type_stubs::*};
        
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeconstructibleTileProxy {

}

impl Prototype for DeconstructibleTileProxy {
    const TYPE: Option<&'static str> = Some("deconstructible-tile-proxy");
}


