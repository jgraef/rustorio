use serde::{Deserialize, Serialize};

use crate::prototypes::{Prototype, Visitor};
use crate::types::*; // TODO: Import only specific types

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Splitter {
    /// structure :: Animation4Way
    structure: Animation4Way,

    /// structure_animation_movement_cooldown :: uint32 (optional)
    structure_animation_movement_cooldown: Option<u32>,

    /// structure_animation_speed_coefficient :: double (optional)
    structure_animation_speed_coefficient: Option<f64>,

    /// structure_patch :: Animation4Way (optional)
    structure_patch: Option<Animation4Way>,
}

impl Prototype for Splitter {
    const TYPE: Option<&'static str> = Some("splitter");
}
