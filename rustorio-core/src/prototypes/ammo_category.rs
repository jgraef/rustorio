use serde::{Serialize, Deserialize};
        
use super::{Prototype, type_stubs::*};
        
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AmmoCategory {
    /// bonus_gui_order :: Order (optional)
    bonus_gui_order: Option<Order>,    

}

impl Prototype for AmmoCategory {
    const TYPE: Option<&'static str> = Some("ammo-category");
}


