use serde::{Serialize, Deserialize};
        
use super::{Prototype, type_stubs::*};
        
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ResourceCategory {

}

impl Prototype for ResourceCategory {
    const TYPE: Option<&'static str> = Some("resource-category");
}


