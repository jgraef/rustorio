use serde::{Serialize, Deserialize};
        
use super::{Prototype, type_stubs::*};
        
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Fish {
    /// pictures :: SpriteVariations
    pictures: SpriteVariations,    

}

impl Prototype for Fish {
    const TYPE: Option<&'static str> = Some("fish");
}


