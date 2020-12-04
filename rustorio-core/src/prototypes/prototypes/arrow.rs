use serde::{Deserialize, Serialize};

use crate::types::*; // TODO: Import only specific types

use rustorio_data_derive::{FromLuaTable, Prototype};


#[derive(Clone, Debug, Serialize, Deserialize, FromLuaTable, Prototype)]
#[prototype(inherits="super::entity::Entity")]
pub struct Arrow {
    /// arrow_picture :: Sprite
    arrow_picture: Sprite,

    /// blinking :: bool (optional)
    blinking: Option<bool>,

    /// circle_picture :: Sprite (optional)
    circle_picture: Option<Sprite>,
}
