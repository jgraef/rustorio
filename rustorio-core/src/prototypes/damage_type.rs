use serde::{Deserialize, Serialize};

use super::Prototype;
use crate::types::*; // TODO: Import only specific types

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DamageType {
    /// hidden :: bool (optional)
    hidden: Option<bool>,
}

impl Prototype for DamageType {
    const TYPE: Option<&'static str> = Some("damage-type");
}
