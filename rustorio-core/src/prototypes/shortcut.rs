use serde::{Serialize, Deserialize};
        
use super::{Prototype, type_stubs::*};
        
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Shortcut {
    /// action :: string
    action: String,    

    /// icon :: Sprite
    icon: Sprite,    

    /// associated_control_input :: string (optional)
    associated_control_input: Option<String>,    

    /// disabled_icon :: Sprite (optional)
    disabled_icon: Option<Sprite>,    

    /// disabled_small_icon :: Sprite (optional)
    disabled_small_icon: Option<Sprite>,    

    /// item_to_create :: string (optional)
    item_to_create: Option<String>,    

    /// small_icon :: Sprite (optional)
    small_icon: Option<Sprite>,    

    /// style :: string (optional)
    style: Option<String>,    

    /// technology_to_unlock :: string (optional)
    technology_to_unlock: Option<String>,    

    /// toggleable :: bool (optional)
    toggleable: Option<bool>,    

}

impl Prototype for Shortcut {
    const TYPE: Option<&'static str> = Some("shortcut");
}


