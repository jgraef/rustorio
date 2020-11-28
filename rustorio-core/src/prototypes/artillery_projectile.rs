use serde::{Serialize, Deserialize};
        
use super::{Prototype, type_stubs::*};
        
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ArtilleryProjectile {
    /// reveal_map :: bool
    reveal_map: bool,    

    /// action :: Trigger (optional)
    action: Option<Trigger>,    

    /// chart_picture :: Sprite (optional)
    chart_picture: Option<Sprite>,    

    /// final_action :: Trigger (optional)
    final_action: Option<Trigger>,    

    /// height_from_ground :: float (optional)
    height_from_ground: Option<f32>,    

    /// picture :: Sprite (optional)
    picture: Option<Sprite>,    

    /// rotatable :: bool (optional)
    rotatable: Option<bool>,    

    /// shadow :: Sprite (optional)
    shadow: Option<Sprite>,    

}

impl Prototype for ArtilleryProjectile {
    const TYPE: Option<&'static str> = Some("artillery-projectile");
}


