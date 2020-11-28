use serde::{Serialize, Deserialize};
        
use super::{Prototype, type_stubs::*};
        
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TransportBeltConnectable {
    /// speed :: double
    speed: f64,    

    /// animation_speed_coefficient :: double (optional)
    animation_speed_coefficient: Option<f64>,    

    /// belt_animation_set :: table (optional)
    belt_animation_set: Option<Vec<Todo>>,

    /// belt_horizontal :: Animation (optional)
    belt_horizontal: Option<Animation>,    

    /// belt_vertical :: Animation (optional)
    belt_vertical: Option<Animation>,    

    /// ending_bottom :: Animation (optional)
    ending_bottom: Option<Animation>,    

    /// ending_patch :: Sprite4Way (optional)
    ending_patch: Option<Sprite4Way>,    

    /// ending_side :: Animation (optional)
    ending_side: Option<Animation>,    

    /// ending_top :: Animation (optional)
    ending_top: Option<Animation>,    

    /// ends_with_stopper :: bool (optional)
    ends_with_stopper: Option<bool>,    

    /// starting_bottom :: Animation (optional)
    starting_bottom: Option<Animation>,    

    /// starting_side :: Animation (optional)
    starting_side: Option<Animation>,    

    /// starting_top :: Animation (optional)
    starting_top: Option<Animation>,    

}

impl Prototype for TransportBeltConnectable {
    const TYPE: Option<&'static str> = Some("None");
}


