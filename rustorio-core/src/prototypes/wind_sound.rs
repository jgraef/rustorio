use serde::{Serialize, Deserialize};
        
use super::{Prototype, type_stubs::*};
        
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WindSound {
    /// name :: string
    name: String,    

    /// sound :: Sound
    sound: Sound,    

    /// type :: string
    r#type: String,    

}

impl Prototype for WindSound {
    const TYPE: Option<&'static str> = Some("wind-sound");
}


