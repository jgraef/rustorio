use serde::{Deserialize, Serialize};

use crate::prototypes::{Prototype, Visitor};
use crate::types::*; // TODO: Import only specific types

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Furnace {
    /// result_inventory_size :: ItemStackIndex
    result_inventory_size: ItemStackIndex,

    /// source_inventory_size :: ItemStackIndex
    source_inventory_size: ItemStackIndex,
}

impl Prototype for Furnace {
    const TYPE: Option<&'static str> = Some("furnace");
}
