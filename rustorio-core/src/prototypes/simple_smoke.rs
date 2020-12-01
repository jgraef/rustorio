use serde::{Deserialize, Serialize};

use super::Prototype;
use crate::types::*; // TODO: Import only specific types

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SimpleSmoke {}

impl Prototype for SimpleSmoke {
    const TYPE: Option<&'static str> = Some("smoke");
}
