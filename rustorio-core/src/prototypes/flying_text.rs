use serde::{Serialize, Deserialize};
        
use super::{Prototype, type_stubs::*};
        
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FlyingText {
    /// speed :: float
    speed: f32,    

    /// time_to_live :: uint32
    time_to_live: u32,    

    /// text_alignment :: string (optional)
    text_alignment: Option<String>,    

}

impl Prototype for FlyingText {
    const TYPE: Option<&'static str> = Some("flying-text");
}


