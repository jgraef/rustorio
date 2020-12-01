use serde::{Deserialize, Serialize};

use super::Prototype;
use crate::types::*; // TODO: Import only specific types

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CopyPasteTool {
    /// cuts :: bool (optional)
    cuts: Option<bool>,
}

impl Prototype for CopyPasteTool {
    const TYPE: Option<&'static str> = Some("copy-paste-tool");
}
