use serde::{Deserialize, Serialize};

use super::Prototype;
use crate::types::*; // TODO: Import only specific types

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConstructWithRobotsAchievement {
    /// limited_to_one_game :: bool
    limited_to_one_game: bool,

    /// amount :: uint32 (optional)
    amount: Option<u32>,

    /// more_than_manually :: bool (optional)
    more_than_manually: Option<bool>,
}

impl Prototype for ConstructWithRobotsAchievement {
    const TYPE: Option<&'static str> = Some("construct-with-robots-achievement");
}
