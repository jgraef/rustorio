use serde::{Serialize, Deserialize};
        
use super::{Prototype, type_stubs::*};
        
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HighlightBoxEntity {

}

impl Prototype for HighlightBoxEntity {
    const TYPE: Option<&'static str> = Some("highlight-box");
}


