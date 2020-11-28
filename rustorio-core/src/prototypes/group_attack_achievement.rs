use serde::{Serialize, Deserialize};
        
use super::{Prototype, type_stubs::*};
        
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GroupAttackAchievement {
    /// amount :: uint32 (optional)
    amount: Option<u32>,    

}

impl Prototype for GroupAttackAchievement {
    const TYPE: Option<&'static str> = Some("group-attack-achievement");
}


