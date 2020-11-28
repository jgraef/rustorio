use serde::{Serialize, Deserialize};
        
use super::{Prototype, type_stubs::*};
        
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BlueprintItem {

}

impl Prototype for BlueprintItem {
    const TYPE: Option<&'static str> = Some("blueprint");
}


