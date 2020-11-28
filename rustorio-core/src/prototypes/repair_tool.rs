use serde::{Serialize, Deserialize};
        
use super::{Prototype, type_stubs::*};
        
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


