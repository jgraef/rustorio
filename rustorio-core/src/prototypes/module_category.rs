use serde::{Serialize, Deserialize};
        
use super::{Prototype, type_stubs::*};
        
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ModuleCategory {

}

impl Prototype for ModuleCategory {
    const TYPE: Option<&'static str> = Some("module-category");
}


