use serde::{Deserialize, Serialize};

use crate::types::*; // TODO: Import only specific types

use rustorio_data_derive::{FromLuaTable, Prototype};


#[derive(Clone, Debug, Serialize, Deserialize, FromLuaTable, Prototype)]
#[prototype(inherits="super::prototype_base::PrototypeBase")]
pub struct AutoplaceControl {
    /// category :: string
    category: String,

    /// richness :: bool (optional)
    richness: Option<bool>,
}
