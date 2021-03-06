use serde::{Deserialize, Serialize};

use crate::prototypes::{Prototype, Visitor};
use crate::types::*; // TODO: Import only specific types

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeconstructibleTileProxy {}

impl Prototype for DeconstructibleTileProxy {
    const TYPE: Option<&'static str> = Some("deconstructible-tile-proxy");
}
