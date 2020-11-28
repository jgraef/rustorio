use serde::{Serialize, Deserialize};
        
use super::{Prototype, type_stubs::*};
        
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RailRemnants {
    /// bending_type :: string
    bending_type: String,    

    /// pictures :: table
    pictures: Vec<Todo>,

}

impl Prototype for RailRemnants {
    const TYPE: Option<&'static str> = Some("rail-remnants");
}


