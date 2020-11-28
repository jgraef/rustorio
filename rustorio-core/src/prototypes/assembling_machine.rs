use serde::{Serialize, Deserialize};
        
use super::{Prototype, type_stubs::*};
        
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AssemblingMachine {
    /// fixed_recipe :: string (optional)
    fixed_recipe: Option<String>,    

    /// gui_title_key :: string (optional)
    gui_title_key: Option<String>,    

    /// ingredient_count :: uint8 (optional)
    ingredient_count: Option<u8>,    

}

impl Prototype for AssemblingMachine {
    const TYPE: Option<&'static str> = Some("assembling-machine");
}


