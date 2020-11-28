use serde::{Serialize, Deserialize};
        
use super::{Prototype, type_stubs::*};
        
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Capsule {
    /// capsule_action :: CapsuleAction
    capsule_action: CapsuleAction,    

    /// radius_color :: Color (optional)
    radius_color: Option<Color>,    

}

impl Prototype for Capsule {
    const TYPE: Option<&'static str> = Some("capsule");
}


