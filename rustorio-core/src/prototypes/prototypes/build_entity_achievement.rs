use serde::{Deserialize, Serialize};

use crate::prototypes::{Prototype, Visitor};
use crate::types::*; // TODO: Import only specific types

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BuildEntityAchievement {
    /// to_build :: string
    to_build: String,

    /// amount :: uint32 (optional)
    amount: Option<u32>,

    /// limited_to_one_game :: bool (optional)
    limited_to_one_game: Option<bool>,

    /// until_second :: uint32 (optional)
    until_second: Option<u32>,
}

impl Prototype for BuildEntityAchievement {
    const TYPE: Option<&'static str> = Some("build-entity-achievement");
}
