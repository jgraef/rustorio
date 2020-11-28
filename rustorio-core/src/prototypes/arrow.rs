use serde::{Serialize, Deserialize};
        
use super::{Prototype, type_stubs::*};
        
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Arrow {
    /// arrow_picture :: Sprite
    arrow_picture: Sprite,    

    /// blinking :: bool (optional)
    blinking: Option<bool>,    

    /// circle_picture :: Sprite (optional)
    circle_picture: Option<Sprite>,    

}

impl Prototype for Arrow {
    const TYPE: Option<&'static str> = Some("arrow");
}


