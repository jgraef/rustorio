use serde::{Deserialize, Serialize};

use super::Prototype;
use crate::types::*; // TODO: Import only specific types

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MouseCursor {
    /// name :: string
    name: String,

    /// type :: string
    r#type: String,

    /// filename :: FileName (optional)
    filename: Option<FileName>,

    /// hot_pixel_x :: int16 (optional)
    hot_pixel_x: Option<i16>,

    /// hot_pixel_y :: int16 (optional)
    hot_pixel_y: Option<i16>,

    /// system_cursor :: string (optional)
    system_cursor: Option<String>,
}

impl Prototype for MouseCursor {
    const TYPE: Option<&'static str> = Some("mouse-cursor");
}
