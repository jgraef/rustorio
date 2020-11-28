use serde::{Serialize, Deserialize};
        
use super::{Prototype, type_stubs::*};
        
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DontCraftManuallyAchievement {
    /// amount :: MaterialAmountType
    amount: MaterialAmountType,    

}

impl Prototype for DontCraftManuallyAchievement {
    const TYPE: Option<&'static str> = Some("dont-craft-manually-achievement");
}


