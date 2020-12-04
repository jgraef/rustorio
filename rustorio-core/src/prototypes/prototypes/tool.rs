use serde::{Deserialize, Serialize};

use crate::prototypes::{Prototype, Visitor};
use crate::types::*; // TODO: Import only specific types

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Tool {
    /// durability :: double (optional)
    durability: Option<f64>,

    /// durability_description_key :: string (optional)
    durability_description_key: Option<String>,

    /// durability_description_value :: string (optional)
    durability_description_value: Option<String>,

    /// infinite :: bool (optional)
    infinite: Option<bool>,
}

impl Prototype for Tool {
    const TYPE: Option<&'static str> = Some("tool");
}
