use serde::{Deserialize, Serialize};

use crate::prototypes::{Prototype, Visitor};
use crate::types::*; // TODO: Import only specific types

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct KillAchievement {
    /// amount :: uint32 (optional)
    amount: Option<u32>,

    /// damage_type :: string (optional)
    damage_type: Option<String>,

    /// in_vehicle :: bool (optional)
    in_vehicle: Option<bool>,

    /// personally :: bool (optional)
    personally: Option<bool>,

    /// to_kill :: string (optional)
    to_kill: Option<String>,

    /// type_to_kill :: string (optional)
    type_to_kill: Option<String>,
}

impl Prototype for KillAchievement {
    const TYPE: Option<&'static str> = Some("kill-achievement");
}
