use serde::{Deserialize, Serialize};

use super::Prototype;
use crate::types::*; // TODO: Import only specific types

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StraightRail {
    /// bending_type :: string (optional)
    bending_type: Option<String>,
}

impl Prototype for StraightRail {
    const TYPE: Option<&'static str> = Some("straight-rail");
}
