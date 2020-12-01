use serde::{Deserialize, Serialize};

use super::Prototype;
use crate::types::*; // TODO: Import only specific types

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FlameThrowerExplosion {
    /// damage :: DamagePrototype
    damage: DamagePrototype,

    /// slow_down_factor :: double
    slow_down_factor: f64,
}

impl Prototype for FlameThrowerExplosion {
    const TYPE: Option<&'static str> = Some("flame-thrower-explosion");
}
