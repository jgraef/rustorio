use serde::{Serialize, Deserialize};
        
use super::{Prototype, type_stubs::*};
        
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Font {
    /// from :: string
    from: String,    

    /// name :: string
    name: String,    

    /// size :: int32
    size: i32,    

    /// type :: string
    r#type: String,    

    /// border :: bool (optional)
    border: Option<bool>,    

    /// border_color :: Color (optional)
    border_color: Option<Color>,    

    /// filtered :: bool (optional)
    filtered: Option<bool>,    

    /// spacing :: float (optional)
    spacing: Option<f32>,    

}

impl Prototype for Font {
    const TYPE: Option<&'static str> = Some("font");
}


