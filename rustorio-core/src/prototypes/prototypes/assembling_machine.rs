use serde::{Deserialize, Serialize};

use crate::types::*; // TODO: Import only specific types

use rustorio_data_derive::{FromLuaTable, Prototype};


#[derive(Clone, Debug, Serialize, Deserialize, FromLuaTable, Prototype)]
#[prototype(inherits="super::crafting_machine::CraftingMachine")]
pub struct AssemblingMachine {
    /// fixed_recipe :: string (optional)
    fixed_recipe: Option<String>,

    /// gui_title_key :: string (optional)
    gui_title_key: Option<String>,

    /// ingredient_count :: uint8 (optional)
    ingredient_count: Option<u8>,
}
