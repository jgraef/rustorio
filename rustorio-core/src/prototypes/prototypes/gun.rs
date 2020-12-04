use serde::{Deserialize, Serialize};

use crate::prototypes::{Prototype, Visitor};
use crate::types::*; // TODO: Import only specific types

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Gun {
    /// attack_parameters :: AttackParameters
    attack_parameters: AttackParameters,
}

impl Prototype for Gun {
    const TYPE: Option<&'static str> = Some("gun");
}
