use serde::{Serialize, Deserialize};
        
use super::{Prototype, type_stubs::*};
        
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ItemWithEntityData {

}

impl Prototype for ItemWithEntityData {
    const TYPE: Option<&'static str> = Some("item-with-entity-data");
}


