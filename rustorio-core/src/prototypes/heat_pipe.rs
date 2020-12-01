use serde::{Deserialize, Serialize};

use super::Prototype;
use crate::types::*; // TODO: Import only specific types

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HeatPipe {
    /// connection_sprites :: ConnectableEntityGraphics
    connection_sprites: ConnectableEntityGraphics,

    /// heat_buffer :: HeatBuffer
    heat_buffer: HeatBuffer,

    /// heat_glow_sprites :: ConnectableEntityGraphics
    heat_glow_sprites: ConnectableEntityGraphics,
}

impl Prototype for HeatPipe {
    const TYPE: Option<&'static str> = Some("heat-pipe");
}
