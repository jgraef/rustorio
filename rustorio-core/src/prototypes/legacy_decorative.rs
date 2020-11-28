use serde::{Serialize, Deserialize};
        
use super::{Prototype, type_stubs::*};
        
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LegacyDecorative {

}

impl Prototype for LegacyDecorative {
    const TYPE: Option<&'static str> = Some("decorative");
}


