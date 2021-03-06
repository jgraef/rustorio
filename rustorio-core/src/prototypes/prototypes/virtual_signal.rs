use serde::{Deserialize, Serialize};

use crate::prototypes::{Prototype, Visitor};
use crate::types::*; // TODO: Import only specific types

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VirtualSignal {
    /// icons, icon, icon_size (IconSpecification) :: IconSpecification
    icon_spec: IconSpecification,

    /// special_signal :: bool (optional)
    special_signal: Option<bool>,

    /// subgroup :: string (optional)
    subgroup: Option<String>,
}

impl Prototype for VirtualSignal {
    const TYPE: Option<&'static str> = Some("virtual-signal");
}
