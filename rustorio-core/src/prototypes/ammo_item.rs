use serde::{Serialize, Deserialize};
        
use super::{Prototype, type_stubs::*};
        
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AmmoItem {
    /// ammo_type :: table of AmmoType or AmmoType
    ammo_type: Todo,

    /// magazine_size :: float (optional)
    magazine_size: Option<f32>,    

    /// reload_time :: float (optional)
    reload_time: Option<f32>,    

}

impl Prototype for AmmoItem {
    const TYPE: Option<&'static str> = Some("ammo");
}

