use serde::{Serialize, Deserialize};
        
use super::{Prototype, type_stubs::*};
        
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SpectatorController {
    /// movement_speed :: double
    movement_speed: f64,    

    /// name :: string
    name: String,    

    /// type :: string
    r#type: String,    

}

impl Prototype for SpectatorController {
    const TYPE: Option<&'static str> = Some("spectator-controller");
}


