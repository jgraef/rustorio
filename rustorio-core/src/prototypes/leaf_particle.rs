use serde::{Deserialize, Serialize};

use super::Prototype;
use crate::types::*; // TODO: Import only specific types

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LeafParticle {}

impl Prototype for LeafParticle {
    const TYPE: Option<&'static str> = Some("leaf-particle");
}
