use serde::{Serialize, Deserialize};
        
use super::{Prototype, type_stubs::*};
        
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EntityParticle {

}

impl Prototype for EntityParticle {
    const TYPE: Option<&'static str> = Some("particle");
}


