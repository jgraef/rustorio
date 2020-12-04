use serde::{Deserialize, Serialize};

use crate::prototypes::{Prototype, Visitor};
use crate::types::*; // TODO: Import only specific types

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
