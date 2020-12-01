use serde::{Deserialize, Serialize};

use super::Prototype;
use crate::types::*; // TODO: Import only specific types

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ModuleCategory {}

impl Prototype for ModuleCategory {
    const TYPE: Option<&'static str> = Some("module-category");
}
