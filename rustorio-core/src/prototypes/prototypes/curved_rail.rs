use serde::{Deserialize, Serialize};

use crate::prototypes::{Prototype, Visitor};
use crate::types::*; // TODO: Import only specific types

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CurvedRail {
    /// bending_type :: string (optional)
    bending_type: Option<String>,
}

impl Prototype for CurvedRail {
    const TYPE: Option<&'static str> = Some("curved-rail");
}
