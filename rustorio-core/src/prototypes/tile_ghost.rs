use serde::{Deserialize, Serialize};

use super::Prototype;
use crate::types::*; // TODO: Import only specific types

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TileGhost {}

impl Prototype for TileGhost {
    const TYPE: Option<&'static str> = Some("tile-ghost");
}
