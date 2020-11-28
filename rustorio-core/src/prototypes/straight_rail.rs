use serde::{Serialize, Deserialize};
        
use super::{Prototype, type_stubs::*};
        
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StraightRail {
    /// bending_type :: string (optional)
    bending_type: Option<String>,    

}

impl Prototype for StraightRail {
    const TYPE: Option<&'static str> = Some("straight-rail");
}


