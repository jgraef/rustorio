use serde::{Deserialize, Serialize};

use crate::types::*; // TODO: Import only specific types

use rustorio_data_derive::{FromLuaTable, Prototype};


#[derive(Clone, Debug, Serialize, Deserialize, FromLuaTable, Prototype)]
#[prototype(inherits="super::turret::Turret")]
pub struct AmmoTurret {
    /// automated_ammo_count :: ItemCountType
    automated_ammo_count: ItemCountType,

    /// inventory_size :: ItemStackIndex
    inventory_size: ItemStackIndex,

    /// entity_info_icon_shift :: vector (optional)
    entity_info_icon_shift: Option<Vector2<f32>>,
}
