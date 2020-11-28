use serde::{Serialize, Deserialize};
        
use super::{Prototype, type_stubs::*};
        
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UtilitySprites {
    /// achievement_label_failed :: Sprite
    achievement_label_failed: Sprite,    

    /// achievement_label_locked :: Sprite
    achievement_label_locked: Sprite,    

    /// achievement_label_unlocked :: Sprite
    achievement_label_unlocked: Sprite,    

    /// achievement_label_unlocked_off :: Sprite
    achievement_label_unlocked_off: Sprite,    

    /// add :: Sprite
    add: Sprite,    

    /// ammo_damage_modifier_icon :: Sprite
    ammo_damage_modifier_icon: Sprite,    

    /// ammo_icon :: Sprite
    ammo_icon: Sprite,    

    /// and_or :: Sprite
    and_or: Sprite,    

    /// area_icon :: Sprite
    area_icon: Sprite,    

    /// arrow_button :: Animation
    arrow_button: Animation,    

    /// artillery_range_modifier_icon :: Sprite
    artillery_range_modifier_icon: Sprite,    

    /// auto_character_logistic_trash_slots_modifier_icon :: Sprite
    auto_character_logistic_trash_slots_modifier_icon: Sprite,    

    /// bar_gray_pip :: Sprite
    bar_gray_pip: Sprite,    

    /// battery :: Sprite
    battery: Sprite,    

    /// battery_indicator :: Sprite
    battery_indicator: Sprite,    

    /// battery_indicator_bar :: Sprite
    battery_indicator_bar: Sprite,    

    /// brush_circle_shape :: Sprite
    brush_circle_shape: Sprite,    

    /// brush_icon :: Sprite
    brush_icon: Sprite,    

    /// brush_square_shape :: Sprite
    brush_square_shape: Sprite,    

    /// cable_editor_icon :: Sprite
    cable_editor_icon: Sprite,    

    /// center :: Sprite
    center: Sprite,    

    /// change_recipe :: Sprite
    change_recipe: Sprite,    

    /// character_additional_mining_categories_modifier_icon :: Sprite
    character_additional_mining_categories_modifier_icon: Sprite,    

    /// character_build_distance_modifier_icon :: Sprite
    character_build_distance_modifier_icon: Sprite,    

    /// character_crafting_speed_modifier_icon :: Sprite
    character_crafting_speed_modifier_icon: Sprite,    

    /// character_health_bonus_modifier_icon :: Sprite
    character_health_bonus_modifier_icon: Sprite,    

    /// character_inventory_slots_bonus_modifier_icon :: Sprite
    character_inventory_slots_bonus_modifier_icon: Sprite,    

    /// character_item_drop_distance_modifier_icon :: Sprite
    character_item_drop_distance_modifier_icon: Sprite,    

    /// character_item_pickup_distance_modifier_icon :: Sprite
    character_item_pickup_distance_modifier_icon: Sprite,    

    /// character_logistic_requests_modifier_icon :: Sprite
    character_logistic_requests_modifier_icon: Sprite,    

    /// character_logistic_slots_modifier_icon :: Sprite
    character_logistic_slots_modifier_icon: Sprite,    

    /// character_logistic_trash_slots_modifier_icon :: Sprite
    character_logistic_trash_slots_modifier_icon: Sprite,    

    /// character_loot_pickup_distance_modifier_icon :: Sprite
    character_loot_pickup_distance_modifier_icon: Sprite,    

    /// character_mining_speed_modifier_icon :: Sprite
    character_mining_speed_modifier_icon: Sprite,    

    /// character_reach_distance_modifier_icon :: Sprite
    character_reach_distance_modifier_icon: Sprite,    

    /// character_resource_reach_distance_modifier_icon :: Sprite
    character_resource_reach_distance_modifier_icon: Sprite,    

    /// character_running_speed_modifier_icon :: Sprite
    character_running_speed_modifier_icon: Sprite,    

    /// check_mark :: Sprite
    check_mark: Sprite,    

    /// check_mark_white :: Sprite
    check_mark_white: Sprite,    

    /// circuit_network_panel_black :: Sprite
    circuit_network_panel_black: Sprite,    

    /// circuit_network_panel_white :: Sprite
    circuit_network_panel_white: Sprite,    

    /// cliff_editor_icon :: Sprite
    cliff_editor_icon: Sprite,    

    /// clock :: Sprite
    clock: Sprite,    

    /// clone :: Sprite
    clone: Sprite,    

    /// clone_editor_icon :: Sprite
    clone_editor_icon: Sprite,    

    /// close_black :: Sprite
    close_black: Sprite,    

    /// close_fat :: Sprite
    close_fat: Sprite,    

    /// close_white :: Sprite
    close_white: Sprite,    

    /// clouds :: Animation
    clouds: Animation,    

    /// collapse :: Sprite
    collapse: Sprite,    

    /// collapse_dark :: Sprite
    collapse_dark: Sprite,    

    /// color_effect :: Sprite
    color_effect: Sprite,    

    /// color_picker :: Sprite
    color_picker: Sprite,    

    /// confirm_slot :: Sprite
    confirm_slot: Sprite,    

    /// construction_radius_visualization :: Sprite
    construction_radius_visualization: Sprite,    

    /// copper_wire :: Sprite
    copper_wire: Sprite,    

    /// covered_chunk :: Sprite
    covered_chunk: Sprite,    

    /// crafting_machine_recipe_not_unlocked :: Sprite
    crafting_machine_recipe_not_unlocked: Sprite,    

    /// cursor_box :: table
    cursor_box: Vec<Todo>,

    /// cursor_icon :: Sprite
    cursor_icon: Sprite,    

    /// custom_tag_in_map_view :: Sprite
    custom_tag_in_map_view: Sprite,    

    /// danger_icon :: Sprite
    danger_icon: Sprite,    

    /// deconstruction_mark :: Sprite
    deconstruction_mark: Sprite,    

    /// deconstruction_time_to_live_modifier_icon :: Sprite
    deconstruction_time_to_live_modifier_icon: Sprite,    

    /// decorative_editor_icon :: Sprite
    decorative_editor_icon: Sprite,    

    /// default_ammo_damage_modifier_icon :: Sprite
    default_ammo_damage_modifier_icon: Sprite,    

    /// default_gun_speed_modifier_icon :: Sprite
    default_gun_speed_modifier_icon: Sprite,    

    /// default_turret_attack_modifier_icon :: Sprite
    default_turret_attack_modifier_icon: Sprite,    

    /// destroyed_icon :: Sprite
    destroyed_icon: Sprite,    

    /// down_arrow :: Sprite
    down_arrow: Sprite,    

    /// downloaded :: Sprite
    downloaded: Sprite,    

    /// downloaded_white :: Sprite
    downloaded_white: Sprite,    

    /// downloading :: Sprite
    downloading: Sprite,    

    /// downloading_white :: Sprite
    downloading_white: Sprite,    

    /// dropdown :: Sprite
    dropdown: Sprite,    

    /// editor_pause :: Sprite
    editor_pause: Sprite,    

    /// editor_play :: Sprite
    editor_play: Sprite,    

    /// editor_selection :: Sprite
    editor_selection: Sprite,    

    /// editor_speed_down :: Sprite
    editor_speed_down: Sprite,    

    /// editor_speed_up :: Sprite
    editor_speed_up: Sprite,    

    /// electricity_icon :: Sprite
    electricity_icon: Sprite,    

    /// electricity_icon_unplugged :: Sprite
    electricity_icon_unplugged: Sprite,    

    /// enemy_force_icon :: Sprite
    enemy_force_icon: Sprite,    

    /// enter :: Sprite
    enter: Sprite,    

    /// entity_editor_icon :: Sprite
    entity_editor_icon: Sprite,    

    /// entity_info_dark_background :: Sprite
    entity_info_dark_background: Sprite,    

    /// equipment_collision :: Sprite
    equipment_collision: Sprite,    

    /// equipment_grid :: Sprite
    equipment_grid: Sprite,    

    /// equipment_slot :: Sprite
    equipment_slot: Sprite,    

    /// expand :: Sprite
    expand: Sprite,    

    /// expand_dark :: Sprite
    expand_dark: Sprite,    

    /// expand_dots :: Sprite
    expand_dots: Sprite,    

    /// expand_dots_white :: Sprite
    expand_dots_white: Sprite,    

    /// explosion_chart_visualization :: Animation
    explosion_chart_visualization: Animation,    

    /// export :: Sprite
    export: Sprite,    

    /// export_slot :: Sprite
    export_slot: Sprite,    

    /// favourite_server_icon :: Sprite
    favourite_server_icon: Sprite,    

    /// fluid_icon :: Sprite
    fluid_icon: Sprite,    

    /// fluid_indication_arrow :: Sprite
    fluid_indication_arrow: Sprite,    

    /// fluid_indication_arrow_both_ways :: Sprite
    fluid_indication_arrow_both_ways: Sprite,    

    /// follower_robot_lifetime_modifier_icon :: Sprite
    follower_robot_lifetime_modifier_icon: Sprite,    

    /// force_editor_icon :: Sprite
    force_editor_icon: Sprite,    

    /// fuel_icon :: Sprite
    fuel_icon: Sprite,    

    /// game_stopped_visualization :: Sprite
    game_stopped_visualization: Sprite,    

    /// ghost_bar_pip :: Sprite
    ghost_bar_pip: Sprite,    

    /// ghost_cursor :: Sprite
    ghost_cursor: Sprite,    

    /// ghost_time_to_live_modifier_icon :: Sprite
    ghost_time_to_live_modifier_icon: Sprite,    

    /// give_item_modifier_icon :: Sprite
    give_item_modifier_icon: Sprite,    

    /// go_to_arrow :: Sprite
    go_to_arrow: Sprite,    

    /// gps_map_icon :: Sprite
    gps_map_icon: Sprite,    

    /// green_circle :: Sprite
    green_circle: Sprite,    

    /// green_dot :: Sprite
    green_dot: Sprite,    

    /// green_wire :: Sprite
    green_wire: Sprite,    

    /// green_wire_hightlight :: Sprite
    green_wire_hightlight: Sprite,    

    /// grey_placement_indicator_leg :: Sprite
    grey_placement_indicator_leg: Sprite,    

    /// grey_rail_signal_placement_indicator :: Sprite
    grey_rail_signal_placement_indicator: Sprite,    

    /// gun_speed_modifier_icon :: Sprite
    gun_speed_modifier_icon: Sprite,    

    /// hand :: Sprite
    hand: Sprite,    

    /// health_bar_green_pip :: Sprite
    health_bar_green_pip: Sprite,    

    /// health_bar_red_pip :: Sprite
    health_bar_red_pip: Sprite,    

    /// health_bar_yellow_pip :: Sprite
    health_bar_yellow_pip: Sprite,    

    /// heat_exchange_indication :: Sprite
    heat_exchange_indication: Sprite,    

    /// hint_arrow_down :: Sprite
    hint_arrow_down: Sprite,    

    /// hint_arrow_left :: Sprite
    hint_arrow_left: Sprite,    

    /// hint_arrow_right :: Sprite
    hint_arrow_right: Sprite,    

    /// hint_arrow_up :: Sprite
    hint_arrow_up: Sprite,    

    /// import :: Sprite
    import: Sprite,    

    /// import_slot :: Sprite
    import_slot: Sprite,    

    /// indication_arrow :: Sprite
    indication_arrow: Sprite,    

    /// indication_line :: Sprite
    indication_line: Sprite,    

    /// inserter_stack_size_bonus_modifier_icon :: Sprite
    inserter_stack_size_bonus_modifier_icon: Sprite,    

    /// item_editor_icon :: Sprite
    item_editor_icon: Sprite,    

    /// laboratory_productivity_modifier_icon :: Sprite
    laboratory_productivity_modifier_icon: Sprite,    

    /// laboratory_speed_modifier_icon :: Sprite
    laboratory_speed_modifier_icon: Sprite,    

    /// left_arrow :: Sprite
    left_arrow: Sprite,    

    /// light_cone :: Sprite
    light_cone: Sprite,    

    /// light_medium :: Sprite
    light_medium: Sprite,    

    /// light_small :: Sprite
    light_small: Sprite,    

    /// line_icon :: Sprite
    line_icon: Sprite,    

    /// logistic_network_panel_black :: Sprite
    logistic_network_panel_black: Sprite,    

    /// logistic_network_panel_white :: Sprite
    logistic_network_panel_white: Sprite,    

    /// logistic_radius_visualization :: Sprite
    logistic_radius_visualization: Sprite,    

    /// lua_snippet_tool_icon :: Sprite
    lua_snippet_tool_icon: Sprite,    

    /// map :: Sprite
    map: Sprite,    

    /// map_exchange_string :: Sprite
    map_exchange_string: Sprite,    

    /// max_failed_attempts_per_tick_per_construction_queue_modifier_icon :: Sprite
    max_failed_attempts_per_tick_per_construction_queue_modifier_icon: Sprite,    

    /// max_successful_attempts_per_tick_per_construction_queue_modifier_icon :: Sprite
    max_successful_attempts_per_tick_per_construction_queue_modifier_icon: Sprite,    

    /// maximum_following_robots_count_modifier_icon :: Sprite
    maximum_following_robots_count_modifier_icon: Sprite,    

    /// medium_gui_arrow :: Sprite
    medium_gui_arrow: Sprite,    

    /// mining_drill_productivity_bonus_modifier_icon :: Sprite
    mining_drill_productivity_bonus_modifier_icon: Sprite,    

    /// missing_icon :: Sprite
    missing_icon: Sprite,    

    /// missing_mod_icon :: Sprite
    missing_mod_icon: Sprite,    

    /// mod_dependency_arrow :: Sprite
    mod_dependency_arrow: Sprite,    

    /// multiplayer_waiting_icon :: Sprite
    multiplayer_waiting_icon: Sprite,    

    /// nature_icon :: Sprite
    nature_icon: Sprite,    

    /// neutral_force_icon :: Sprite
    neutral_force_icon: Sprite,    

    /// no_building_material_icon :: Sprite
    no_building_material_icon: Sprite,    

    /// no_nature_icon :: Sprite
    no_nature_icon: Sprite,    

    /// no_storage_space_icon :: Sprite
    no_storage_space_icon: Sprite,    

    /// none_editor_icon :: Sprite
    none_editor_icon: Sprite,    

    /// not_available :: Sprite
    not_available: Sprite,    

    /// not_enough_construction_robots_icon :: Sprite
    not_enough_construction_robots_icon: Sprite,    

    /// not_enough_repair_packs_icon :: Sprite
    not_enough_repair_packs_icon: Sprite,    

    /// nothing_modifier_icon :: Sprite
    nothing_modifier_icon: Sprite,    

    /// paint_bucket_icon :: Sprite
    paint_bucket_icon: Sprite,    

    /// pause :: Sprite
    pause: Sprite,    

    /// placement_indicator_leg :: Sprite
    placement_indicator_leg: Sprite,    

    /// play :: Sprite
    play: Sprite,    

    /// player_force_icon :: Sprite
    player_force_icon: Sprite,    

    /// pollution_visualization :: Sprite
    pollution_visualization: Sprite,    

    /// preset :: Sprite
    preset: Sprite,    

    /// pump_cannot_connect_icon :: Sprite
    pump_cannot_connect_icon: Sprite,    

    /// questionmark :: Sprite
    questionmark: Sprite,    

    /// quick_bar_count_modifier_icon :: Sprite
    quick_bar_count_modifier_icon: Sprite,    

    /// rail_path_not_possible :: Sprite
    rail_path_not_possible: Sprite,    

    /// rail_planner_indication_arrow :: Sprite
    rail_planner_indication_arrow: Sprite,    

    /// rail_planner_indication_arrow_too_far :: Sprite
    rail_planner_indication_arrow_too_far: Sprite,    

    /// rail_signal_placement_indicator :: Sprite
    rail_signal_placement_indicator: Sprite,    

    /// recharge_icon :: Sprite
    recharge_icon: Sprite,    

    /// red_wire :: Sprite
    red_wire: Sprite,    

    /// red_wire_hightlight :: Sprite
    red_wire_hightlight: Sprite,    

    /// refresh :: Sprite
    refresh: Sprite,    

    /// refresh_white :: Animation
    refresh_white: Animation,    

    /// rename_icon_normal :: Sprite
    rename_icon_normal: Sprite,    

    /// rename_icon_small_black :: Sprite
    rename_icon_small_black: Sprite,    

    /// rename_icon_small_white :: Sprite
    rename_icon_small_white: Sprite,    

    /// reset :: Sprite
    reset: Sprite,    

    /// reset_white :: Sprite
    reset_white: Sprite,    

    /// resource_editor_icon :: Sprite
    resource_editor_icon: Sprite,    

    /// right_arrow :: Sprite
    right_arrow: Sprite,    

    /// robot_slot :: Sprite
    robot_slot: Sprite,    

    /// scripting_editor_icon :: Sprite
    scripting_editor_icon: Sprite,    

    /// search_black :: Sprite
    search_black: Sprite,    

    /// search_icon :: Sprite
    search_icon: Sprite,    

    /// search_white :: Sprite
    search_white: Sprite,    

    /// set_bar_slot :: Sprite
    set_bar_slot: Sprite,    

    /// shield_bar_pip :: Sprite
    shield_bar_pip: Sprite,    

    /// shoot_cursor_green :: Sprite
    shoot_cursor_green: Sprite,    

    /// shoot_cursor_red :: Sprite
    shoot_cursor_red: Sprite,    

    /// short_indication_line :: Sprite
    short_indication_line: Sprite,    

    /// show_electric_network_in_map_view :: Sprite
    show_electric_network_in_map_view: Sprite,    

    /// show_electric_network_in_map_view_black :: Sprite
    show_electric_network_in_map_view_black: Sprite,    

    /// show_logistics_network_in_map_view :: Sprite
    show_logistics_network_in_map_view: Sprite,    

    /// show_logistics_network_in_map_view_black :: Sprite
    show_logistics_network_in_map_view_black: Sprite,    

    /// show_player_names_in_map_view :: Sprite
    show_player_names_in_map_view: Sprite,    

    /// show_player_names_in_map_view_black :: Sprite
    show_player_names_in_map_view_black: Sprite,    

    /// show_pollution_in_map_view :: Sprite
    show_pollution_in_map_view: Sprite,    

    /// show_pollution_in_map_view_black :: Sprite
    show_pollution_in_map_view_black: Sprite,    

    /// show_train_station_names_in_map_view :: Sprite
    show_train_station_names_in_map_view: Sprite,    

    /// show_train_station_names_in_map_view_black :: Sprite
    show_train_station_names_in_map_view_black: Sprite,    

    /// show_turret_range_in_map_view :: Sprite
    show_turret_range_in_map_view: Sprite,    

    /// show_turret_range_in_map_view_black :: Sprite
    show_turret_range_in_map_view_black: Sprite,    

    /// shuffle :: Sprite
    shuffle: Sprite,    

    /// side_menu_achievements_hover_icon :: Sprite
    side_menu_achievements_hover_icon: Sprite,    

    /// side_menu_achievements_icon :: Sprite
    side_menu_achievements_icon: Sprite,    

    /// side_menu_blueprint_library_hover_icon :: Sprite
    side_menu_blueprint_library_hover_icon: Sprite,    

    /// side_menu_blueprint_library_icon :: Sprite
    side_menu_blueprint_library_icon: Sprite,    

    /// side_menu_bonus_hover_icon :: Sprite
    side_menu_bonus_hover_icon: Sprite,    

    /// side_menu_bonus_icon :: Sprite
    side_menu_bonus_icon: Sprite,    

    /// side_menu_map_hover_icon :: Sprite
    side_menu_map_hover_icon: Sprite,    

    /// side_menu_map_icon :: Sprite
    side_menu_map_icon: Sprite,    

    /// side_menu_menu_hover_icon :: Sprite
    side_menu_menu_hover_icon: Sprite,    

    /// side_menu_menu_icon :: Sprite
    side_menu_menu_icon: Sprite,    

    /// side_menu_production_hover_icon :: Sprite
    side_menu_production_hover_icon: Sprite,    

    /// side_menu_production_icon :: Sprite
    side_menu_production_icon: Sprite,    

    /// side_menu_train_hover_icon :: Sprite
    side_menu_train_hover_icon: Sprite,    

    /// side_menu_train_icon :: Sprite
    side_menu_train_icon: Sprite,    

    /// side_menu_tutorials_hover_icon :: Sprite
    side_menu_tutorials_hover_icon: Sprite,    

    /// side_menu_tutorials_icon :: Sprite
    side_menu_tutorials_icon: Sprite,    

    /// slot :: Sprite
    slot: Sprite,    

    /// slot_icon_ammo :: Sprite
    slot_icon_ammo: Sprite,    

    /// slot_icon_armor :: Sprite
    slot_icon_armor: Sprite,    

    /// slot_icon_fuel :: Sprite
    slot_icon_fuel: Sprite,    

    /// slot_icon_gun :: Sprite
    slot_icon_gun: Sprite,    

    /// slot_icon_module :: Sprite
    slot_icon_module: Sprite,    

    /// slot_icon_resource :: Sprite
    slot_icon_resource: Sprite,    

    /// slot_icon_result :: Sprite
    slot_icon_result: Sprite,    

    /// slot_icon_robot :: Sprite
    slot_icon_robot: Sprite,    

    /// slot_icon_robot_material :: Sprite
    slot_icon_robot_material: Sprite,    

    /// small_gui_arrow :: Sprite
    small_gui_arrow: Sprite,    

    /// spawn_flag :: Sprite
    spawn_flag: Sprite,    

    /// speed_down :: Sprite
    speed_down: Sprite,    

    /// speed_up :: Sprite
    speed_up: Sprite,    

    /// spray_icon :: Sprite
    spray_icon: Sprite,    

    /// stack_inserter_capacity_bonus_modifier_icon :: Sprite
    stack_inserter_capacity_bonus_modifier_icon: Sprite,    

    /// station_name :: Sprite
    station_name: Sprite,    

    /// stop :: Sprite
    stop: Sprite,    

    /// surface_editor_icon :: Sprite
    surface_editor_icon: Sprite,    

    /// sync_mods :: Sprite
    sync_mods: Sprite,    

    /// tick_custom :: Sprite
    tick_custom: Sprite,    

    /// tick_once :: Sprite
    tick_once: Sprite,    

    /// tick_sixty :: Sprite
    tick_sixty: Sprite,    

    /// tile_editor_icon :: Sprite
    tile_editor_icon: Sprite,    

    /// tile_ghost_cursor :: Sprite
    tile_ghost_cursor: Sprite,    

    /// time_editor_icon :: Sprite
    time_editor_icon: Sprite,    

    /// too_far :: Sprite
    too_far: Sprite,    

    /// too_far_from_roboport_icon :: Sprite
    too_far_from_roboport_icon: Sprite,    

    /// track_button :: Sprite
    track_button: Sprite,    

    /// train_braking_force_bonus_modifier_icon :: Sprite
    train_braking_force_bonus_modifier_icon: Sprite,    

    /// train_stop_disabled_in_map_view :: Sprite
    train_stop_disabled_in_map_view: Sprite,    

    /// train_stop_in_map_view :: Sprite
    train_stop_in_map_view: Sprite,    

    /// train_stop_placement_indicator :: Sprite
    train_stop_placement_indicator: Sprite,    

    /// trash :: Sprite
    trash: Sprite,    

    /// trash_bin :: Sprite
    trash_bin: Sprite,    

    /// trash_white :: Sprite
    trash_white: Sprite,    

    /// turret_attack_modifier_icon :: Sprite
    turret_attack_modifier_icon: Sprite,    

    /// underground_pipe_connection :: Sprite
    underground_pipe_connection: Sprite,    

    /// underground_remove_belts :: Sprite
    underground_remove_belts: Sprite,    

    /// underground_remove_pipes :: Sprite
    underground_remove_pipes: Sprite,    

    /// unlock_recipe_modifier_icon :: Sprite
    unlock_recipe_modifier_icon: Sprite,    

    /// upgrade_blueprint :: Sprite
    upgrade_blueprint: Sprite,    

    /// upgrade_mark :: Sprite
    upgrade_mark: Sprite,    

    /// variations_tool_icon :: Sprite
    variations_tool_icon: Sprite,    

    /// warning :: Sprite
    warning: Sprite,    

    /// warning_icon :: Sprite
    warning_icon: Sprite,    

    /// warning_white :: Sprite
    warning_white: Sprite,    

    /// white_mask :: Sprite
    white_mask: Sprite,    

    /// white_square :: Sprite
    white_square: Sprite,    

    /// wire_shadow :: Sprite
    wire_shadow: Sprite,    

    /// worker_robot_battery_modifier_icon :: Sprite
    worker_robot_battery_modifier_icon: Sprite,    

    /// worker_robot_speed_modifier_icon :: Sprite
    worker_robot_speed_modifier_icon: Sprite,    

    /// worker_robot_storage_modifier_icon :: Sprite
    worker_robot_storage_modifier_icon: Sprite,    

    /// zoom_to_world_blueprint_enabled_modifier_icon :: Sprite
    zoom_to_world_blueprint_enabled_modifier_icon: Sprite,    

    /// zoom_to_world_deconstruction_planner_enabled_modifier_icon :: Sprite
    zoom_to_world_deconstruction_planner_enabled_modifier_icon: Sprite,    

    /// zoom_to_world_enabled_modifier_icon :: Sprite
    zoom_to_world_enabled_modifier_icon: Sprite,    

    /// zoom_to_world_ghost_building_enabled_modifier_icon :: Sprite
    zoom_to_world_ghost_building_enabled_modifier_icon: Sprite,    

    /// zoom_to_world_selection_tool_enabled_modifier_icon :: Sprite
    zoom_to_world_selection_tool_enabled_modifier_icon: Sprite,    

    /// zoom_to_world_upgrade_planner_enabled_modifier_icon :: Sprite
    zoom_to_world_upgrade_planner_enabled_modifier_icon: Sprite,    

}

impl Prototype for UtilitySprites {
    const TYPE: Option<&'static str> = Some("utility-sprites");
}


