use serde::{Deserialize, Serialize};

use crate::prototypes::{Prototype, Visitor};
use crate::types::*; // TODO: Import only specific types

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpgradeItem {
    /// mapper_count :: ItemStackIndex (optional)
    mapper_count: Option<ItemStackIndex>,
}

impl Prototype for UpgradeItem {
    const TYPE: Option<&'static str> = Some("upgrade-item");
}
