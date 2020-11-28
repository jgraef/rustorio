use serde::{Serialize, Deserialize};
        
use super::{Prototype, type_stubs::*};
        
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SelectionTool {
    /// alt_selection_color :: Color
    alt_selection_color: Color,    

    /// alt_selection_cursor_box_type :: string
    alt_selection_cursor_box_type: String,    

    /// alt_selection_mode :: table of string
    alt_selection_mode: Vec<String>,

    /// selection_color :: Color
    selection_color: Color,    

    /// selection_cursor_box_type :: string
    selection_cursor_box_type: String,    

    /// selection_mode :: table of string
    selection_mode: Vec<String>,

    /// alt_entity_filter_mode :: string (optional)
    alt_entity_filter_mode: Option<String>,    

    /// alt_entity_filters :: table of string (optional)
    alt_entity_filters: Option<Vec<String>>,

    /// alt_entity_type_filters :: table of string (optional)
    alt_entity_type_filters: Option<Vec<String>>,

    /// alt_tile_filter_mode :: string (optional)
    alt_tile_filter_mode: Option<String>,    

    /// alt_tile_filters :: table of string (optional)
    alt_tile_filters: Option<Vec<String>>,

    /// always_include_tiles :: bool (optional)
    always_include_tiles: Option<bool>,    

    /// entity_filter_mode :: string (optional)
    entity_filter_mode: Option<String>,    

    /// entity_filters :: table of string (optional)
    entity_filters: Option<Vec<String>>,

    /// entity_type_filters :: table of string (optional)
    entity_type_filters: Option<Vec<String>>,

    /// mouse_cursor :: string (optional)
    mouse_cursor: Option<String>,    

    /// show_in_library :: bool (optional)
    show_in_library: Option<bool>,    

    /// tile_filter_mode :: string (optional)
    tile_filter_mode: Option<String>,    

    /// tile_filters :: table of string (optional)
    tile_filters: Option<Vec<String>>,

}

impl Prototype for SelectionTool {
    const TYPE: Option<&'static str> = Some("selection-tool");
}


