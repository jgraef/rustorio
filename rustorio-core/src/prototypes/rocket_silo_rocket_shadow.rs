use serde::{Serialize, Deserialize};
        
use super::{Prototype, type_stubs::*};
        
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RocketSiloRocketShadow {

}

impl Prototype for RocketSiloRocketShadow {
    const TYPE: Option<&'static str> = Some("rocket-silo-rocket-shadow");
}


