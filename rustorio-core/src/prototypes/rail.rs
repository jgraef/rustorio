use serde::{Serialize, Deserialize};
        
use super::{Prototype, type_stubs::*};
        
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Rail {
    /// pictures :: table
    pictures: Vec<Todo>,

    /// walking_sound :: Sound (optional)
    walking_sound: Option<Sound>,    

}

impl Prototype for Rail {
    const TYPE: Option<&'static str> = Some("None");
}


