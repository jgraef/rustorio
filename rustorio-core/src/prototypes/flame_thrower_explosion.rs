use serde::{Serialize, Deserialize};
        
use super::{Prototype, type_stubs::*};
        
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FlameThrowerExplosion {
    /// damage :: DamagePrototype
    damage: DamagePrototype,    

    /// slow_down_factor :: double
    slow_down_factor: f64,    

}

impl Prototype for FlameThrowerExplosion {
    const TYPE: Option<&'static str> = Some("flame-thrower-explosion");
}


