use serde::{Deserialize, Serialize};

use crate::prototypes::{Prototype, Visitor};
use crate::types::*; // TODO: Import only specific types

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Fish {
    /// pictures :: SpriteVariations
    pictures: SpriteVariations,
}

impl Prototype for Fish {
    const TYPE: Option<&'static str> = Some("fish");
}
