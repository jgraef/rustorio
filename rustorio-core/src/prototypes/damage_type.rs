use serde::{Serialize, Deserialize};
        
use super::{Prototype, type_stubs::*};
        
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DamageType {
    /// hidden :: bool (optional)
    hidden: Option<bool>,    

}

impl Prototype for DamageType {
    const TYPE: Option<&'static str> = Some("damage-type");
}


