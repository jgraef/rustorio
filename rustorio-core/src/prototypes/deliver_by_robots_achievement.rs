use serde::{Deserialize, Serialize};

use super::Prototype;
use crate::types::*; // TODO: Import only specific types

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeliverByRobotsAchievement {
    /// amount :: MaterialAmountType
    amount: MaterialAmountType,
}

impl Prototype for DeliverByRobotsAchievement {
    const TYPE: Option<&'static str> = Some("deliver-by-robots-achievement");
}
