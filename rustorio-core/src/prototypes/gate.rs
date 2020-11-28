use serde::{Serialize, Deserialize};
        
use super::{Prototype, type_stubs::*};
        
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Gate {
    /// activation_distance :: double
    activation_distance: f64,    

    /// close_sound :: Sound
    close_sound: Sound,    

    /// horizontal_animation :: Animation
    horizontal_animation: Animation,    

    /// horizontal_rail_animation_left :: Animation
    horizontal_rail_animation_left: Animation,    

    /// horizontal_rail_animation_right :: Animation
    horizontal_rail_animation_right: Animation,    

    /// horizontal_rail_base :: Animation
    horizontal_rail_base: Animation,    

    /// open_sound :: Sound
    open_sound: Sound,    

    /// opening_speed :: float
    opening_speed: f32,    

    /// timeout_to_close :: uint32
    timeout_to_close: u32,    

    /// vertical_animation :: Animation
    vertical_animation: Animation,    

    /// vertical_rail_animation_left :: Animation
    vertical_rail_animation_left: Animation,    

    /// vertical_rail_animation_right :: Animation
    vertical_rail_animation_right: Animation,    

    /// vertical_rail_base :: Animation
    vertical_rail_base: Animation,    

    /// wall_patch :: Animation
    wall_patch: Animation,    

    /// fadeout_interval :: uint32 (optional)
    fadeout_interval: Option<u32>,    

    /// opened_collision_mask :: CollisionMask (optional)
    opened_collision_mask: Option<CollisionMask>,    

}

impl Prototype for Gate {
    const TYPE: Option<&'static str> = Some("gate");
}


