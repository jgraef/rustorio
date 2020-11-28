use serde::{Serialize, Deserialize};
        
use super::{Prototype, type_stubs::*};
        
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


