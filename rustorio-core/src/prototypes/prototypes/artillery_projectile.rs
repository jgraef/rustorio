use serde::{Deserialize, Serialize};

use crate::types::*; // TODO: Import only specific types

use rustorio_data_derive::{FromLuaTable, Prototype};


#[derive(Clone, Debug, Serialize, Deserialize, FromLuaTable, Prototype)]
#[prototype(inherits="super::entity::Entity")]
pub struct ArtilleryProjectile {
    /// reveal_map :: bool
    reveal_map: bool,

    /// action :: Trigger (optional)
    action: Option<Trigger>,

    /// chart_picture :: Sprite (optional)
    chart_picture: Option<Sprite>,

    /// final_action :: Trigger (optional)
    final_action: Option<Trigger>,

    /// height_from_ground :: float (optional)
    height_from_ground: Option<f32>,

    /// picture :: Sprite (optional)
    picture: Option<Sprite>,

    /// rotatable :: bool (optional)
    rotatable: Option<bool>,

    /// shadow :: Sprite (optional)
    shadow: Option<Sprite>,
}
