use serde::{Serialize, Deserialize};
        
use super::{Prototype, type_stubs::*};
        
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpgradeItem {
    /// mapper_count :: ItemStackIndex (optional)
    mapper_count: Option<ItemStackIndex>,    

}

impl Prototype for UpgradeItem {
    const TYPE: Option<&'static str> = Some("upgrade-item");
}


