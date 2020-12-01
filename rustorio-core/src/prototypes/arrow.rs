use serde::{Deserialize, Serialize};

use super::Prototype;
use crate::types::*; // TODO: Import only specific types

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Arrow {
    /// arrow_picture :: Sprite
    arrow_picture: Sprite,

    /// blinking :: bool (optional)
    blinking: Option<bool>,

    /// circle_picture :: Sprite (optional)
    circle_picture: Option<Sprite>,
}

impl Prototype for Arrow {
    const TYPE: Option<&'static str> = Some("arrow");
}
