use serde::{Deserialize, Serialize};

use crate::prototypes::{Prototype, Visitor};
use crate::types::*; // TODO: Import only specific types

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RepairTool {
    /// speed :: float
    speed: f32,

    /// repair_result :: Trigger (optional)
    repair_result: Option<Trigger>,
}

impl Prototype for RepairTool {
    const TYPE: Option<&'static str> = Some("repair-tool");
}
