use serde::{Deserialize, Serialize};

use super::Prototype;
use crate::types::*; // TODO: Import only specific types

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CargoWagon {
    /// inventory_size :: ItemStackIndex
    inventory_size: ItemStackIndex,
}

impl Prototype for CargoWagon {
    const TYPE: Option<&'static str> = Some("cargo-wagon");
}
