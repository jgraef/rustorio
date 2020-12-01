use serde::{Deserialize, Serialize};

use super::Prototype;
use crate::types::*; // TODO: Import only specific types

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NoiseLayer {}

impl Prototype for NoiseLayer {
    const TYPE: Option<&'static str> = Some("noise-layer");
}
