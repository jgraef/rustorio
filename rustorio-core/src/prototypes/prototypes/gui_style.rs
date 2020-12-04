use serde::{Deserialize, Serialize};

use crate::prototypes::{Prototype, Visitor};
use crate::types::*; // TODO: Import only specific types

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GuiStyle {
    /// default_sprite_priority :: SpritePriority
    default_sprite_priority: SpritePriority,

    /// default_sprite_scale :: double
    default_sprite_scale: f64,

    /// default_tileset :: FileName
    default_tileset: FileName,

    /// name :: string
    name: String,

    /// type :: string
    r#type: String,
}

impl Prototype for GuiStyle {
    const TYPE: Option<&'static str> = Some("gui-style");
}
