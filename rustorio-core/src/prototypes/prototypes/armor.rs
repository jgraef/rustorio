use serde::{Deserialize, Serialize};

use crate::types::*; // TODO: Import only specific types

use rustorio_data_derive::{FromLuaTable, Prototype};


#[derive(Clone, Debug, Serialize, Deserialize, FromLuaTable, Prototype)]
#[prototype(inherits="super::tool::Tool")]
pub struct Armor {
    /// equipment_grid :: string (optional)
    equipment_grid: Option<String>,

    /// inventory_size_bonus :: ItemStackIndex (optional)
    inventory_size_bonus: Option<ItemStackIndex>,

    /// resistances :: Resistances (optional)
    resistances: Option<Resistances>,
}
