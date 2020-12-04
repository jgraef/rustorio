use serde::{Deserialize, Serialize};

use crate::prototypes::{Prototype, Visitor};
use crate::types::*; // TODO: Import only specific types

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeconstructionItem {
    /// entity_filter_count :: Types/ItemStackIndex (optional)
    entity_filter_count: Option<ItemStackIndex>,

    /// tile_filter_count :: Types/ItemStackIndex (optional)
    tile_filter_count: Option<ItemStackIndex>,
}

impl Prototype for DeconstructionItem {
    const TYPE: Option<&'static str> = Some("deconstruction-item");
}
