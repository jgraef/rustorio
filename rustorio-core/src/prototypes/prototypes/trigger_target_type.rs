use serde::{Deserialize, Serialize};

use crate::prototypes::{Prototype, Visitor};
use crate::types::*; // TODO: Import only specific types

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TriggerTargetType {
    /// name :: string
    name: String,

    /// type :: string
    r#type: String,
}

impl Prototype for TriggerTargetType {
    const TYPE: Option<&'static str> = Some("trigger-target-type");
}
