use serde::{Deserialize, Serialize};

use super::Prototype;
use crate::types::*; // TODO: Import only specific types

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BlueprintItem {}

impl Prototype for BlueprintItem {
    const TYPE: Option<&'static str> = Some("blueprint");
}
