use serde::{Serialize, Deserialize};
        
use super::{Prototype, type_stubs::*};
        
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ItemRequestProxy {
    /// picture :: Sprite
    picture: Sprite,    

    /// use_target_entity_alert_icon_shift :: bool (optional)
    use_target_entity_alert_icon_shift: Option<bool>,    

}

impl Prototype for ItemRequestProxy {
    const TYPE: Option<&'static str> = Some("item-request-proxy");
}


