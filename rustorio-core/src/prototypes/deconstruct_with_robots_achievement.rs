use serde::{Deserialize, Serialize};

use super::Prototype;
use crate::types::*; // TODO: Import only specific types

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeconstructWithRobotsAchievement {
    /// amount :: uint32
    amount: u32,
}

impl Prototype for DeconstructWithRobotsAchievement {
    const TYPE: Option<&'static str> = Some("deconstruct-with-robots-achievement");
}
