use serde::{Serialize, Deserialize};
        
use super::{Prototype, type_stubs::*};
        
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UndergroundBelt {
    /// max_distance :: uint8
    max_distance: u8,    

    /// structure :: table
    structure: Vec<Todo>,

    /// underground_sprite :: Sprite
    underground_sprite: Sprite,    

    /// underground_remove_belts_sprite :: Sprite (optional)
    underground_remove_belts_sprite: Option<Sprite>,    

}

impl Prototype for UndergroundBelt {
    const TYPE: Option<&'static str> = Some("underground-belt");
}


