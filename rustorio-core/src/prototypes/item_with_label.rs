use serde::{Serialize, Deserialize};
        
use super::{Prototype, type_stubs::*};
        
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


