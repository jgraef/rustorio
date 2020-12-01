use serde::{Deserialize, Serialize};

use super::Prototype;
use crate::types::*; // TODO: Import only specific types

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MapGenPresets {
    /// name :: string
    name: String,

    /// type :: string
    r#type: String,
}

impl Prototype for MapGenPresets {
    const TYPE: Option<&'static str> = Some("map-gen-presets");
}
