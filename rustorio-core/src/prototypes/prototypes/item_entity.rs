use serde::{Deserialize, Serialize};

use crate::prototypes::{Prototype, Visitor};
use crate::types::*; // TODO: Import only specific types

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ItemEntity {}

impl Prototype for ItemEntity {
    const TYPE: Option<&'static str> = Some("item-entity");
}
