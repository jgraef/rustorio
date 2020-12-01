use serde::{Deserialize, Serialize};

use super::Prototype;
use crate::types::*; // TODO: Import only specific types

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AmmoCategory {
    /// bonus_gui_order :: Order (optional)
    bonus_gui_order: Option<Order>,
}

impl Prototype for AmmoCategory {
    const TYPE: Option<&'static str> = Some("ammo-category");
}
