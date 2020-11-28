use serde::{Serialize, Deserialize};
        
use super::{Prototype, type_stubs::*};
        
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AmmoTurret {
    /// automated_ammo_count :: ItemCountType
    automated_ammo_count: ItemCountType,    

    /// inventory_size :: ItemStackIndex
    inventory_size: ItemStackIndex,    

    /// entity_info_icon_shift :: vector (optional)
    entity_info_icon_shift: Option<Vector2<f32>>,    

}

impl Prototype for AmmoTurret {
    const TYPE: Option<&'static str> = Some("ammo-turret");
}


