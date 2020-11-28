use serde::{Serialize, Deserialize};
        
use super::{Prototype, type_stubs::*};
        
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SimpleSmoke {

}

impl Prototype for SimpleSmoke {
    const TYPE: Option<&'static str> = Some("smoke");
}


