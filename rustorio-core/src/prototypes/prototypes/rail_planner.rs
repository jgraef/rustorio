use serde::{Deserialize, Serialize};

use crate::prototypes::{Prototype, Visitor};
use crate::types::*; // TODO: Import only specific types

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RailPlanner {
    /// curved_rail :: string
    curved_rail: String,

    /// straight_rail :: string
    straight_rail: String,
}

impl Prototype for RailPlanner {
    const TYPE: Option<&'static str> = Some("rail-planner");
}
