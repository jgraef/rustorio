use serde::{Deserialize, Serialize};

use super::Prototype;
use crate::types::*; // TODO: Import only specific types

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RocketSiloRocketShadow {}

impl Prototype for RocketSiloRocketShadow {
    const TYPE: Option<&'static str> = Some("rocket-silo-rocket-shadow");
}
