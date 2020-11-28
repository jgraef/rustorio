use serde::{Serialize, Deserialize};
        
use super::{Prototype, type_stubs::*};
        
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CombatRobotCountAchievement {
    /// count :: uint32 (optional)
    count: Option<u32>,    

}

impl Prototype for CombatRobotCountAchievement {
    const TYPE: Option<&'static str> = Some("combat-robot-count");
}


