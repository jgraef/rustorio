use serde::{Deserialize, Serialize};

use super::Prototype;
use crate::types::*; // TODO: Import only specific types

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RailSignal {}

impl Prototype for RailSignal {
    const TYPE: Option<&'static str> = Some("rail-signal");
}
