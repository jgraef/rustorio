use serde::{Deserialize, Serialize};

use crate::prototypes::{Prototype, Visitor};
use crate::types::*; // TODO: Import only specific types

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Market {
    /// picture :: Sprite
    picture: Sprite,

    /// allow_access_to_all_forces :: bool (optional)
    allow_access_to_all_forces: Option<bool>,
}

impl Prototype for Market {
    const TYPE: Option<&'static str> = Some("market");
}
