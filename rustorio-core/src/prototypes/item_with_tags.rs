use serde::{Serialize, Deserialize};
        
use super::{Prototype, type_stubs::*};
        
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ItemWithTags {

}

impl Prototype for ItemWithTags {
    const TYPE: Option<&'static str> = Some("item-with-tags");
}


