use serde::{Serialize, Deserialize};
        
use super::{Prototype, type_stubs::*};
        
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PlayerPort {
    /// animation :: Animation
    animation: Animation,    

}

impl Prototype for PlayerPort {
    const TYPE: Option<&'static str> = Some("player-port");
}


