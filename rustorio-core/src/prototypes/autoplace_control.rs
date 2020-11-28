use serde::{Serialize, Deserialize};
        
use super::{Prototype, type_stubs::*};
        
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AutoplaceControl {
    /// category :: string
    category: String,    

    /// richness :: bool (optional)
    richness: Option<bool>,    

}

impl Prototype for AutoplaceControl {
    const TYPE: Option<&'static str> = Some("autoplace-control");
}


