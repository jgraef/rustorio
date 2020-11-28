use serde::{Serialize, Deserialize};
        
use super::{Prototype, type_stubs::*};
        
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CurvedRail {
    /// bending_type :: string (optional)
    bending_type: Option<String>,    

}

impl Prototype for CurvedRail {
    const TYPE: Option<&'static str> = Some("curved-rail");
}


