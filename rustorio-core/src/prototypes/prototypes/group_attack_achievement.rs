use serde::{Deserialize, Serialize};

use crate::prototypes::{Prototype, Visitor};
use crate::types::*; // TODO: Import only specific types

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GroupAttackAchievement {
    /// amount :: uint32 (optional)
    amount: Option<u32>,
}

impl Prototype for GroupAttackAchievement {
    const TYPE: Option<&'static str> = Some("group-attack-achievement");
}
