use serde::{Serialize, Deserialize};
        
use super::{Prototype, type_stubs::*};
        
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeconstructWithRobotsAchievement {
    /// amount :: uint32
    amount: u32,    

}

impl Prototype for DeconstructWithRobotsAchievement {
    const TYPE: Option<&'static str> = Some("deconstruct-with-robots-achievement");
}


