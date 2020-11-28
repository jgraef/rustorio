use serde::{Serialize, Deserialize};
        
use super::{Prototype, type_stubs::*};
        
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PrototypeBase {
    /// name :: string
    name: String,    

    /// type :: string
    r#type: String,    

    /// localised_description :: LocalisedString (optional)
    localised_description: Option<LocalisedString>,    

    /// localised_name :: LocalisedString (optional)
    localised_name: Option<LocalisedString>,    

    /// order :: Order (optional)
    order: Option<Order>,    

}

impl Prototype for PrototypeBase {
    const TYPE: Option<&'static str> = Some("None");
}


