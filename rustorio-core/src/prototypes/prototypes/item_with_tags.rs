use serde::{Deserialize, Serialize};

use crate::prototypes::{Prototype, Visitor};
use crate::types::*; // TODO: Import only specific types

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ItemWithTags {}

impl Prototype for ItemWithTags {
    const TYPE: Option<&'static str> = Some("item-with-tags");
}
