use serde::{Deserialize, Serialize};

use crate::prototypes::{Prototype, Visitor};
use crate::types::*; // TODO: Import only specific types

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LandMine {
    /// picture_safe :: Sprite
    picture_safe: Sprite,

    /// picture_set :: Sprite
    picture_set: Sprite,

    /// trigger_radius :: double
    trigger_radius: f64,

    /// action :: Trigger (optional)
    action: Option<Trigger>,

    /// ammo_category :: string (optional)
    ammo_category: Option<String>,

    /// force_die_on_attack :: bool (optional)
    force_die_on_attack: Option<bool>,

    /// picture_set_enemy :: Sprite (optional)
    picture_set_enemy: Option<Sprite>,

    /// timeout :: uint32 (optional)
    timeout: Option<u32>,

    /// trigger_force :: ForceCondition (optional)
    trigger_force: Option<ForceCondition>,
}

impl Prototype for LandMine {
    const TYPE: Option<&'static str> = Some("land-mine");
}
