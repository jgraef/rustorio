use serde::{Serialize, Deserialize};
        
use super::{Prototype, type_stubs::*};
        
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UtilityConstants {
    /// artillery_range_visualization_color :: Color
    artillery_range_visualization_color: Color,    

    /// bonus_gui_ordering :: table
    bonus_gui_ordering: Vec<Todo>,

    /// building_buildable_tint :: Color
    building_buildable_tint: Color,    

    /// building_buildable_too_far_tint :: Color
    building_buildable_too_far_tint: Color,    

    /// building_ignorable_tint :: Color
    building_ignorable_tint: Color,    

    /// building_no_tint :: Color
    building_no_tint: Color,    

    /// building_not_buildable_tint :: Color
    building_not_buildable_tint: Color,    

    /// capsule_range_visualization_color :: Color
    capsule_range_visualization_color: Color,    

    /// chart :: table
    chart: Vec<Todo>,

    /// clipboard_history_size :: uint32
    clipboard_history_size: u32,    

    /// color_filters :: table (array) of tables
    color_filters: Vec<Vec<Todo>>,

    /// daytime_color_lookup :: DaytimeColorLookupTable
    daytime_color_lookup: DaytimeColorLookupTable,    

    /// deconstruct_mark_tint :: Color
    deconstruct_mark_tint: Color,    

    /// default_alert_icon_scale :: float
    default_alert_icon_scale: f32,    

    /// default_enemy_force_color :: Color
    default_enemy_force_color: Color,    

    /// default_other_force_color :: Color
    default_other_force_color: Color,    

    /// default_player_force_color :: Color
    default_player_force_color: Color,    

    /// default_scorch_mark_color :: Color
    default_scorch_mark_color: Color,    

    /// disabled_recipe_slot_background_tint :: Color
    disabled_recipe_slot_background_tint: Color,    

    /// disabled_recipe_slot_tint :: Color
    disabled_recipe_slot_tint: Color,    

    /// enabled_recipe_slot_tint :: Color
    enabled_recipe_slot_tint: Color,    

    /// entity_button_background_color :: Color
    entity_button_background_color: Color,    

    /// entity_renderer_search_box_limits :: table
    entity_renderer_search_box_limits: Vec<Todo>,

    /// equipment_default_background_border_color :: Color
    equipment_default_background_border_color: Color,    

    /// equipment_default_background_color :: Color
    equipment_default_background_color: Color,    

    /// equipment_default_grabbed_background_color :: Color
    equipment_default_grabbed_background_color: Color,    

    /// filter_outline_color :: Color
    filter_outline_color: Color,    

    /// forced_enabled_recipe_slot_background_tint :: Color
    forced_enabled_recipe_slot_background_tint: Color,    

    /// ghost_tint :: Color
    ghost_tint: Color,    

    /// icon_shadow_color :: Color
    icon_shadow_color: Color,    

    /// icon_shadow_inset :: float
    icon_shadow_inset: f32,    

    /// icon_shadow_radius :: float
    icon_shadow_radius: f32,    

    /// icon_shadow_sharpness :: float
    icon_shadow_sharpness: f32,    

    /// item_outline_color :: Color
    item_outline_color: Color,    

    /// item_outline_inset :: float
    item_outline_inset: f32,    

    /// item_outline_radius :: float
    item_outline_radius: f32,    

    /// item_outline_sharpness :: float
    item_outline_sharpness: f32,    

    /// light_renderer_search_distance_limit :: uint8
    light_renderer_search_distance_limit: u8,    

    /// main_menu_background_image_location :: FileName
    main_menu_background_image_location: FileName,    

    /// manual_rail_building_reach_modifier :: double
    manual_rail_building_reach_modifier: f64,    

    /// map_editor :: table
    map_editor: Vec<Todo>,

    /// max_terrain_building_size :: uint8
    max_terrain_building_size: u8,    

    /// medium_area_size :: float
    medium_area_size: f32,    

    /// missing_preview_sprite_location :: FileName
    missing_preview_sprite_location: FileName,    

    /// player_colors :: table (array) of tables
    player_colors: Vec<Vec<Todo>>,

    /// rail_segment_colors :: table (array) of Color
    rail_segment_colors: Vec<Color>,    

    /// recipe_step_limit :: uint32
    recipe_step_limit: u32,    

    /// script_command_console_chat_color :: Color
    script_command_console_chat_color: Color,    

    /// server_command_console_chat_color :: Color
    server_command_console_chat_color: Color,    

    /// small_area_size :: float
    small_area_size: f32,    

    /// tile_ghost_tint :: Color
    tile_ghost_tint: Color,    

    /// train_inactivity_wait_condition_default :: uint32
    train_inactivity_wait_condition_default: u32,    

    /// train_path_finding :: table
    train_path_finding: Vec<Todo>,

    /// train_temporary_stop_wait_time :: uint32
    train_temporary_stop_wait_time: u32,    

    /// train_time_wait_condition_default :: uint32
    train_time_wait_condition_default: u32,    

    /// tree_leaf_distortion_distortion_far :: vector
    tree_leaf_distortion_distortion_far: Vector2<f32>,    

    /// tree_leaf_distortion_distortion_near :: vector
    tree_leaf_distortion_distortion_near: Vector2<f32>,    

    /// tree_leaf_distortion_speed_far :: vector
    tree_leaf_distortion_speed_far: Vector2<f32>,    

    /// tree_leaf_distortion_speed_near :: vector
    tree_leaf_distortion_speed_near: Vector2<f32>,    

    /// tree_leaf_distortion_strength_far :: vector
    tree_leaf_distortion_strength_far: Vector2<f32>,    

    /// tree_leaf_distortion_strength_near :: vector
    tree_leaf_distortion_strength_near: Vector2<f32>,    

    /// tree_shadow_roughness :: float
    tree_shadow_roughness: f32,    

    /// tree_shadow_speed :: float
    tree_shadow_speed: f32,    

    /// turret_range_visualization_color :: Color
    turret_range_visualization_color: Color,    

    /// unit_group_pathfind_resolution :: int8
    unit_group_pathfind_resolution: i8,    

    /// zoom_to_world_can_use_nightvision :: bool
    zoom_to_world_can_use_nightvision: bool,    

    /// zoom_to_world_daytime_color_lookup :: DaytimeColorLookupTable
    zoom_to_world_daytime_color_lookup: DaytimeColorLookupTable,    

    /// zoom_to_world_effect_strength :: float
    zoom_to_world_effect_strength: f32,    

    /// default_alert_icon_scale_by_type :: table (array) of string to float (optional)
    default_alert_icon_scale_by_type: Option<Vec<String>>,

    /// default_alert_icon_shift_by_type :: table (array) of string to vector (optional)
    default_alert_icon_shift_by_type: Option<Vec<String>>,

    /// default_trigger_target_mask_by_type :: table (array) of string to TriggerTargetMask (optional)
    default_trigger_target_mask_by_type: Option<Vec<String>>,

}

impl Prototype for UtilityConstants {
    const TYPE: Option<&'static str> = Some("utility-constants");
}


