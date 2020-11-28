use serde::{Serialize, Deserialize};
        
use super::{Prototype, type_stubs::*};
        
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeliverByRobotsAchievement {
    /// amount :: MaterialAmountType
    amount: MaterialAmountType,    

}

impl Prototype for DeliverByRobotsAchievement {
    const TYPE: Option<&'static str> = Some("deliver-by-robots-achievement");
}


