use serde::{Serialize, Deserialize};
        
use super::{Prototype, type_stubs::*};
        
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BlueprintBook {
    /// show_in_library :: bool (optional)
    show_in_library: Option<bool>,    

}

impl Prototype for BlueprintBook {
    const TYPE: Option<&'static str> = Some("blueprint-book");
}


