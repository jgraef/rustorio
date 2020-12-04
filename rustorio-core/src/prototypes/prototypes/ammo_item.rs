use serde::{Deserialize, Serialize};

use crate::types::*; // TODO: Import only specific types

use rustorio_data_derive::{FromLuaTable, Prototype};


#[derive(Clone, Debug, Serialize, Deserialize, FromLuaTable, Prototype)]
#[prototype(inherits="super::item::Item")]
pub struct AmmoItem {
    /// ammo_type :: table of AmmoType or AmmoType
    ammo_type: Todo,

    /// magazine_size :: float (optional)
    magazine_size: Option<f32>,

    /// reload_time :: float (optional)
    reload_time: Option<f32>,
}
