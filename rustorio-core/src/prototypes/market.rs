use serde::{Serialize, Deserialize};
        
use super::{Prototype, type_stubs::*};
        
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Market {
    /// picture :: Sprite
    picture: Sprite,    

    /// allow_access_to_all_forces :: bool (optional)
    allow_access_to_all_forces: Option<bool>,    

}

impl Prototype for Market {
    const TYPE: Option<&'static str> = Some("market");
}


