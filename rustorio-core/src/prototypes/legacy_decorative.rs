use serde::{Deserialize, Serialize};

use super::Prototype;
use crate::types::*; // TODO: Import only specific types

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LegacyDecorative {}

impl Prototype for LegacyDecorative {
    const TYPE: Option<&'static str> = Some("decorative");
}
