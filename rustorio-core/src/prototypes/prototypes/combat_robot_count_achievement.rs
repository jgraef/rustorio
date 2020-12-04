use serde::{Deserialize, Serialize};

use crate::prototypes::{Prototype, Visitor};
use crate::types::*; // TODO: Import only specific types

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CombatRobotCountAchievement {
    /// count :: uint32 (optional)
    count: Option<u32>,
}

impl Prototype for CombatRobotCountAchievement {
    const TYPE: Option<&'static str> = Some("combat-robot-count");
}
