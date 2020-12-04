use serde::{Deserialize, Serialize};

use crate::prototypes::{Prototype, Visitor};
use crate::types::*; // TODO: Import only specific types

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EntityGhost {
    /// large_build_sound :: Sound (optional)
    large_build_sound: Option<Sound>,

    /// medium_build_sound :: Sound (optional)
    medium_build_sound: Option<Sound>,
}

impl Prototype for EntityGhost {
    const TYPE: Option<&'static str> = Some("entity-ghost");
}
