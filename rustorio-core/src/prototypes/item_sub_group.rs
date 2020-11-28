use serde::{Serialize, Deserialize};
        
use super::{Prototype, type_stubs::*};
        
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ItemSubGroup {
    /// group :: string
    group: String,    

}

impl Prototype for ItemSubGroup {
    const TYPE: Option<&'static str> = Some("item-subgroup");
}


