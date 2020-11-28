use serde::{Serialize, Deserialize};
        
use super::{Prototype, type_stubs::*};
        
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MiningTool {

}

impl Prototype for MiningTool {
    const TYPE: Option<&'static str> = Some("mining-tool");
}


