use serde::{Serialize, Deserialize};
        
use super::{Prototype, type_stubs::*};
        
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NoiseLayer {

}

impl Prototype for NoiseLayer {
    const TYPE: Option<&'static str> = Some("noise-layer");
}


