use serde::{Deserialize, Serialize};

use crate::prototypes::{Prototype, Visitor};
use crate::types::*; // TODO: Import only specific types

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NamedNoiseExpression {
    /// expression :: NoiseExpression
    expression: NoiseExpression,

    /// intended_property :: string (optional)
    intended_property: Option<String>,
}

impl Prototype for NamedNoiseExpression {
    const TYPE: Option<&'static str> = Some("noise-expression");
}
