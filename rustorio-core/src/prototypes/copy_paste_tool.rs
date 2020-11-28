use serde::{Serialize, Deserialize};
        
use super::{Prototype, type_stubs::*};
        
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CopyPasteTool {
    /// cuts :: bool (optional)
    cuts: Option<bool>,    

}

impl Prototype for CopyPasteTool {
    const TYPE: Option<&'static str> = Some("copy-paste-tool");
}


