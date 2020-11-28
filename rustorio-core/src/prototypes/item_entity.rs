use serde::{Serialize, Deserialize};
        
use super::{Prototype, type_stubs::*};
        
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ItemEntity {

}

impl Prototype for ItemEntity {
    const TYPE: Option<&'static str> = Some("item-entity");
}


