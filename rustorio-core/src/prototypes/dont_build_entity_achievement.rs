use serde::{Deserialize, Serialize};

use super::Prototype;
use crate::types::*; // TODO: Import only specific types

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DontBuildEntityAchievement {
    /// dont_build :: table of string or string
    dont_build: Todo,

    /// amount :: uint32 (optional)
    amount: Option<u32>,
}

impl Prototype for DontBuildEntityAchievement {
    const TYPE: Option<&'static str> = Some("dont-build-entity-achievement");
}
