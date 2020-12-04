use serde::{Deserialize, Serialize};

use crate::prototypes::{Prototype, Visitor};
use crate::types::*; // TODO: Import only specific types

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EntityWithHealth {
    /// alert_when_damaged :: bool (optional)
    alert_when_damaged: Option<bool>,

    /// attack_reaction :: AttackReaction (optional)
    attack_reaction: Option<AttackReaction>,

    /// corpse :: string or table of strings (optional)
    corpse: Option<Todo>,

    /// create_ghost_on_death :: bool (optional)
    create_ghost_on_death: Option<bool>,

    /// damaged_trigger_effect :: TriggerEffect (optional)
    damaged_trigger_effect: Option<TriggerEffect>,

    /// dying_explosion :: ExplosionDefinition or table of ExplosionDefinition (optional)
    dying_explosion: Option<Todo>,

    /// dying_trigger_effect :: TriggerEffect (optional)
    dying_trigger_effect: Option<TriggerEffect>,

    /// healing_per_tick :: float (optional)
    healing_per_tick: Option<f32>,

    /// hide_resistances :: bool (optional)
    hide_resistances: Option<bool>,

    /// integration_patch :: Sprite4Way (optional)
    integration_patch: Option<Sprite4Way>,

    /// integration_patch_render_layer :: RenderLayer (optional)
    integration_patch_render_layer: Option<RenderLayer>,

    /// loot :: Loot (optional)
    loot: Option<Loot>,

    /// max_health :: float (optional)
    max_health: Option<f32>,

    /// random_corpse_variation :: bool (optional)
    random_corpse_variation: Option<bool>,

    /// repair_sound :: Sound (optional)
    repair_sound: Option<Sound>,

    /// repair_speed_modifier :: float (optional)
    repair_speed_modifier: Option<f32>,

    /// resistances :: Resistances (optional)
    resistances: Option<Resistances>,
}

impl Prototype for EntityWithHealth {
    const TYPE: Option<&'static str> = Some("None");
}
