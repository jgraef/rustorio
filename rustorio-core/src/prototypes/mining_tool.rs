use serde::{Deserialize, Serialize};

use super::Prototype;
use crate::types::*; // TODO: Import only specific types

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MiningTool {}

impl Prototype for MiningTool {
    const TYPE: Option<&'static str> = Some("mining-tool");
}
