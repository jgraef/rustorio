use serde::{Deserialize, Serialize};

use crate::prototypes::{Prototype, Visitor};
use crate::types::*; // TODO: Import only specific types

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ItemSubGroup {
    /// group :: string
    group: String,
}

impl Prototype for ItemSubGroup {
    const TYPE: Option<&'static str> = Some("item-subgroup");
}
