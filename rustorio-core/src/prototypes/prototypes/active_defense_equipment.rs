use serde::{Deserialize, Serialize};

use crate::types::*; // TODO: Import only specific types

use rustorio_data_derive::{FromLuaTable, Prototype};


#[derive(Clone, Debug, Serialize, Deserialize, FromLuaTable, Prototype)]
#[prototype(inherits="super::equipment::Equipment")]
pub struct ActiveDefenseEquipment {
    /// attack_parameters :: AttackParameters
    attack_parameters: AttackParameters,

    /// automatic :: bool
    automatic: bool,
}
