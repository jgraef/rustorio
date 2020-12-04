use serde::{Deserialize, Serialize};

use crate::prototypes::{Prototype, Visitor};
use crate::types::*; // TODO: Import only specific types

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FlyingText {
    /// speed :: float
    speed: f32,

    /// time_to_live :: uint32
    time_to_live: u32,

    /// text_alignment :: string (optional)
    text_alignment: Option<String>,
}

impl Prototype for FlyingText {
    const TYPE: Option<&'static str> = Some("flying-text");
}
