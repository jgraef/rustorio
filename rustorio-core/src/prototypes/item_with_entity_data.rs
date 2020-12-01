use serde::{Deserialize, Serialize};

use super::Prototype;
use crate::types::*; // TODO: Import only specific types

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ItemWithEntityData {}

impl Prototype for ItemWithEntityData {
    const TYPE: Option<&'static str> = Some("item-with-entity-data");
}
