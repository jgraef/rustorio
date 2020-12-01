use serde::{Deserialize, Serialize};

use super::Prototype;
use crate::types::*; // TODO: Import only specific types

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AssemblingMachine {
    /// fixed_recipe :: string (optional)
    fixed_recipe: Option<String>,

    /// gui_title_key :: string (optional)
    gui_title_key: Option<String>,

    /// ingredient_count :: uint8 (optional)
    ingredient_count: Option<u8>,
}

impl Prototype for AssemblingMachine {
    const TYPE: Option<&'static str> = Some("assembling-machine");
}
