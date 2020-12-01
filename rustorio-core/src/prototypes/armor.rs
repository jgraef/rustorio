use serde::{Deserialize, Serialize};

use super::Prototype;
use crate::types::*; // TODO: Import only specific types

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Armor {
    /// equipment_grid :: string (optional)
    equipment_grid: Option<String>,

    /// inventory_size_bonus :: ItemStackIndex (optional)
    inventory_size_bonus: Option<ItemStackIndex>,

    /// resistances :: Resistances (optional)
    resistances: Option<Resistances>,
}

impl Prototype for Armor {
    const TYPE: Option<&'static str> = Some("armor");
}
