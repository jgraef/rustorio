use serde::{Serialize, Deserialize};
        
use super::{Prototype, type_stubs::*};
        
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InfinityPipe {
    /// gui_mode :: string (optional)
    gui_mode: Option<String>,    

}

impl Prototype for InfinityPipe {
    const TYPE: Option<&'static str> = Some("infinity-pipe");
}


