use serde::{Deserialize, Serialize};

use crate::prototypes::{Prototype, Visitor};
use crate::types::*; // TODO: Import only specific types

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InfinityPipe {
    /// gui_mode :: string (optional)
    gui_mode: Option<String>,
}

impl Prototype for InfinityPipe {
    const TYPE: Option<&'static str> = Some("infinity-pipe");
}
