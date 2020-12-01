use serde::{Deserialize, Serialize};

use super::Prototype;
use crate::types::*; // TODO: Import only specific types

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PlayerDamagedAchievement {
    /// minimum_damage :: float
    minimum_damage: f32,

    /// should_survive :: bool
    should_survive: bool,

    /// type_of_dealer :: string (optional)
    type_of_dealer: Option<String>,
}

impl Prototype for PlayerDamagedAchievement {
    const TYPE: Option<&'static str> = Some("player-damaged-achievement");
}
