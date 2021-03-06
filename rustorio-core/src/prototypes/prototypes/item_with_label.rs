use serde::{Deserialize, Serialize};

use crate::prototypes::{Prototype, Visitor};
use crate::types::*; // TODO: Import only specific types

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ItemWithLabel {
    /// default_label_color :: Color (optional)
    default_label_color: Option<Color>,

    /// draw_label_for_cursor_render :: bool (optional)
    draw_label_for_cursor_render: Option<bool>,
}

impl Prototype for ItemWithLabel {
    const TYPE: Option<&'static str> = Some("item-with-label");
}
