use serde::{Deserialize, Serialize};

use super::Prototype;
use crate::types::*; // TODO: Import only specific types

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FinishTheGameAchievement {
    /// until_second :: uint32 (optional)
    until_second: Option<u32>,
}

impl Prototype for FinishTheGameAchievement {
    const TYPE: Option<&'static str> = Some("finish-the-game-achievement");
}
