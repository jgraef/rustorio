use serde::{Serialize, Deserialize};
        
use super::{Prototype, type_stubs::*};
        
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CombatRobot {
    /// attack_parameters :: AttackParameters
    attack_parameters: AttackParameters,    

    /// idle :: RotatedAnimation
    idle: RotatedAnimation,    

    /// in_motion :: RotatedAnimation
    in_motion: RotatedAnimation,    

    /// shadow_idle :: RotatedAnimation
    shadow_idle: RotatedAnimation,    

    /// shadow_in_motion :: RotatedAnimation
    shadow_in_motion: RotatedAnimation,    

    /// time_to_live :: uint32
    time_to_live: u32,    

    /// destroy_action :: Trigger (optional)
    destroy_action: Option<Trigger>,    

    /// follows_player :: bool (optional)
    follows_player: Option<bool>,    

    /// friction :: double (optional)
    friction: Option<f64>,    

    /// light :: LightDefinition (optional)
    light: Option<LightDefinition>,    

    /// range_from_player :: double (optional)
    range_from_player: Option<f64>,    

}

impl Prototype for CombatRobot {
    const TYPE: Option<&'static str> = Some("combat-robot");
}


