use serde::{Serialize, Deserialize};
        
use super::{Prototype, type_stubs::*};
        
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TrainPathAchievement {
    /// minimum_distance :: double
    minimum_distance: f64,    

}

impl Prototype for TrainPathAchievement {
    const TYPE: Option<&'static str> = Some("train-path-achievement");
}


