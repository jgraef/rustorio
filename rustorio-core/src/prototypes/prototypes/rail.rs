use serde::{Deserialize, Serialize};

use crate::prototypes::{Prototype, Visitor};
use crate::types::*; // TODO: Import only specific types

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Rail {
    /// pictures :: table
    pictures: Vec<Todo>,

    /// walking_sound :: Sound (optional)
    walking_sound: Option<Sound>,
}

impl Prototype for Rail {
    const TYPE: Option<&'static str> = Some("None");
}
