use serde::{Deserialize, Serialize};

use super::Prototype;
use crate::types::*; // TODO: Import only specific types

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InfinityContainer {
    /// erase_contents_when_mined :: bool
    erase_contents_when_mined: bool,

    /// gui_mode :: string (optional)
    gui_mode: Option<String>,
}

impl Prototype for InfinityContainer {
    const TYPE: Option<&'static str> = Some("infinity-container");
}
