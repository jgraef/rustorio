use serde::{Serialize, Deserialize};
        
use super::{Prototype, type_stubs::*};
        
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Projectile {
    /// acceleration :: double
    acceleration: f64,    

    /// animation :: Animation
    animation: Animation,    

    /// action :: Trigger (optional)
    action: Option<Trigger>,    

    /// direction_only :: bool (optional)
    direction_only: Option<bool>,    

    /// enable_drawing_with_mask :: bool (optional)
    enable_drawing_with_mask: Option<bool>,    

    /// final_action :: Trigger (optional)
    final_action: Option<Trigger>,    

    /// force_condition :: ForceCondition (optional)
    force_condition: Option<ForceCondition>,    

    /// height :: double (optional)
    height: Option<f64>,    

    /// hit_at_collision_position :: bool (optional)
    hit_at_collision_position: Option<bool>,    

    /// hit_collision_mask :: CollisionMask (optional)
    hit_collision_mask: Option<CollisionMask>,    

    /// light :: LightDefinition (optional)
    light: Option<LightDefinition>,    

    /// max_speed :: double (optional)
    max_speed: Option<f64>,    

    /// piercing_damage :: float (optional)
    piercing_damage: Option<f32>,    

    /// rotatable :: bool (optional)
    rotatable: Option<bool>,    

    /// shadow :: Animation (optional)
    shadow: Option<Animation>,    

    /// smoke :: Array of SmokeSource (optional)
    smoke: Option<Vec<SmokeSource>>,    

    /// turn_speed :: float (optional)
    turn_speed: Option<f32>,    

}

impl Prototype for Projectile {
    const TYPE: Option<&'static str> = Some("projectile");
}


