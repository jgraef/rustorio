use serde::{Deserialize, Serialize};

use crate::prototypes::{Prototype, Visitor};
use crate::types::*; // TODO: Import only specific types

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PlayerPort {
    /// animation :: Animation
    animation: Animation,
}

impl Prototype for PlayerPort {
    const TYPE: Option<&'static str> = Some("player-port");
}
