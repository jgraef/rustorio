use serde::{Deserialize, Serialize};

use crate::types::{self, *}; // TODO: Import only specific types

use rustorio_data_derive::{FromLuaTable, Prototype};


#[derive(Clone, Debug, Serialize, Deserialize, FromLuaTable, Prototype)]
#[prototype(inherits="super::prototype_base::PrototypeBase")]
pub struct Animation {
    /// name :: string
    name: String,

    /// type :: string
    r#type: String,

    #[lua(flatten)]
    animation: types::Animation,
}
