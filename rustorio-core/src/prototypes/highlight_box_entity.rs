use serde::{Deserialize, Serialize};

use super::Prototype;
use crate::types::*; // TODO: Import only specific types

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HighlightBoxEntity {}

impl Prototype for HighlightBoxEntity {
    const TYPE: Option<&'static str> = Some("highlight-box");
}
