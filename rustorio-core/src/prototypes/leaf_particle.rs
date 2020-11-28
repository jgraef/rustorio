use serde::{Serialize, Deserialize};
        
use super::{Prototype, type_stubs::*};
        
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LeafParticle {

}

impl Prototype for LeafParticle {
    const TYPE: Option<&'static str> = Some("leaf-particle");
}


