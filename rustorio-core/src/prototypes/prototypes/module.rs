use serde::{Deserialize, Serialize};

use crate::prototypes::{Prototype, Visitor};
use crate::types::*; // TODO: Import only specific types

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Module {
    /// category :: string
    category: String,

    /// effect :: Effect
    effect: Effect,

    /// tier :: uint32
    tier: u32,

    /// art_style :: string (optional)
    art_style: Option<String>,

    /// beacon_tint :: table of Color (optional)
    beacon_tint: Option<Vec<Color>>,

    /// limitation :: table of string (optional)
    limitation: Option<Vec<String>>,

    /// limitation_message_key :: string (optional)
    limitation_message_key: Option<String>,

    /// requires_beacon_alt_mode :: bool (optional)
    requires_beacon_alt_mode: Option<bool>,
}

impl Prototype for Module {
    const TYPE: Option<&'static str> = Some("module");
}
