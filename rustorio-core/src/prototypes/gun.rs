use serde::{Serialize, Deserialize};
        
use super::{Prototype, type_stubs::*};
        
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Gun {
    /// attack_parameters :: AttackParameters
    attack_parameters: AttackParameters,    

}

impl Prototype for Gun {
    const TYPE: Option<&'static str> = Some("gun");
}


