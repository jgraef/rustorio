use serde::{Deserialize, Serialize};

use super::Prototype;
use crate::types::*; // TODO: Import only specific types

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TrainPathAchievement {
    /// minimum_distance :: double
    minimum_distance: f64,
}

impl Prototype for TrainPathAchievement {
    const TYPE: Option<&'static str> = Some("train-path-achievement");
}
