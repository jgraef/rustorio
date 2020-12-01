use serde::{Deserialize, Serialize};

use super::Prototype;
use crate::types::*; // TODO: Import only specific types

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EntityParticle {}

impl Prototype for EntityParticle {
    const TYPE: Option<&'static str> = Some("particle");
}
