use serde::{Serialize, Deserialize};
        
use super::{Prototype, type_stubs::*};
        
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RecipeCategory {

}

impl Prototype for RecipeCategory {
    const TYPE: Option<&'static str> = Some("recipe-category");
}


