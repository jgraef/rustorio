use serde::{Serialize, Deserialize};
        
use super::{Prototype, type_stubs::*};
        
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RailSignal {

}

impl Prototype for RailSignal {
    const TYPE: Option<&'static str> = Some("rail-signal");
}


