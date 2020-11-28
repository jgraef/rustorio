use serde::{Serialize, Deserialize};
        
use super::{Prototype, type_stubs::*};
        
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RailPlanner {
    /// curved_rail :: string
    curved_rail: String,    

    /// straight_rail :: string
    straight_rail: String,    

}

impl Prototype for RailPlanner {
    const TYPE: Option<&'static str> = Some("rail-planner");
}


