use serde::{Deserialize, Serialize};

use super::Prototype;
use crate::types::*; // TODO: Import only specific types

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AutoplaceControl {
    /// category :: string
    category: String,

    /// richness :: bool (optional)
    richness: Option<bool>,
}

impl Prototype for AutoplaceControl {
    const TYPE: Option<&'static str> = Some("autoplace-control");
}
