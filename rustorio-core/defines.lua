{
  alert_type = {
    custom = 6,
    entity_destroyed = 0,
    entity_under_attack = 1,
    no_material_for_construction = 3,
    no_storage = 7,
    not_enough_construction_robots = 2,
    not_enough_repair_packs = 4,
    train_out_of_fuel = 8,
    turret_fire = 5
  },
  behavior_result = {
    deleted = 3,
    fail = 1,
    in_progress = 0,
    success = 2
  },
  build_check_type = {
    blueprint_ghost = 4,
    ghost_revive = 5,
    manual = 1,
    manual_ghost = 3,
    script = 0,
    script_ghost = 2
  },
  chain_signal_state = {
    all_open = 1,
    none = 0,
    none_open = 3,
    partially_open = 2
  },
  chunk_generated_status = {
    basic_tiles = 20,
    corrected_tiles = 30,
    custom_tiles = 10,
    entities = 50,
    nothing = 0,
    tiles = 40
  },
  circuit_condition_index = {
    arithmetic_combinator = 1,
    constant_combinator = 1,
    decider_combinator = 1,
    inserter_circuit = 1,
    inserter_logistic = 2,
    lamp = 1,
    offshore_pump = 1,
    pump = 1
  },
  circuit_connector_id = {
    accumulator = 1,
    combinator_input = 1,
    combinator_output = 2,
    constant_combinator = 1,
    container = 1,
    electric_pole = 1,
    inserter = 1,
    lamp = 1,
    linked_container = 1,
    offshore_pump = 1,
    programmable_speaker = 1,
    pump = 1,
    rail_chain_signal = 1,
    rail_signal = 1,
    roboport = 1,
    storage_tank = 1,
    wall = 1
  },
  command = {
    attack = 1,
    attack_area = 5,
    build_base = 7,
    compound = 3,
    flee = 8,
    go_to_location = 2,
    group = 4,
    stop = 9,
    wander = 6
  },
  compound_command = {
    logical_and = 0,
    logical_or = 1,
    return_last = 2
  },
  control_behavior = {
    inserter = {
      circuit_mode_of_operation = {
        enable_disable = 0,
        none = 3,
        read_hand_contents = 2,
        set_filters = 1,
        set_stack_size = 4
      },
      hand_read_mode = {
        hold = 1,
        pulse = 0
      }
    },
    lamp = {
      circuit_mode_of_operation = {
        use_colors = 0
      }
    },
    logistic_container = {
      circuit_mode_of_operation = {
        send_contents = 0,
        set_requests = 1
      }
    },
    mining_drill = {
      resource_read_mode = {
        entire_patch = 1,
        this_miner = 0
      }
    },
    transport_belt = {
      content_read_mode = {
        hold = 1,
        pulse = 0
      }
    },
    type = {
      accumulator = 13,
      arithmetic_combinator = 10,
      constant_combinator = 11,
      container = 1,
      decider_combinator = 9,
      generic_on_off = 2,
      inserter = 3,
      lamp = 4,
      logistic_container = 5,
      mining_drill = 16,
      programmable_speaker = 17,
      rail_chain_signal = 18,
      rail_signal = 14,
      roboport = 6,
      storage_tank = 7,
      train_stop = 8,
      transport_belt = 12,
      wall = 15
    }
  },
  controllers = {
    character = 1,
    cutscene = 6,
    editor = 4,
    ghost = 0,
    god = 2,
    spectator = 5
  },
  deconstruction_item = {
    entity_filter_mode = {
      blacklist = 1,
      whitelist = 0
    },
    tile_filter_mode = {
      blacklist = 1,
      whitelist = 0
    },
    tile_selection_mode = {
      always = 1,
      never = 2,
      normal = 0,
      only = 3
    }
  },
  difficulty = {
    easy = 0,
    hard = 2,
    normal = 1
  },
  difficulty_settings = {
    recipe_difficulty = {
      expensive = 1,
      normal = 0
    },
    technology_difficulty = {
      expensive = 1,
      normal = 0
    }
  },
  direction = {
    east = 2,
    north = 0,
    northeast = 1,
    northwest = 7,
    south = 4,
    southeast = 3,
    southwest = 5,
    west = 6
  },
  disconnect_reason = {
    afk = 6,
    banned = 9,
    cannot_keep_up = 5,
    desync_limit_reached = 4,
    dropped = 1,
    kicked = 7,
    kicked_and_deleted = 8,
    quit = 0,
    reconnect = 2,
    switching_servers = 11,
    wrong_input = 3
  },
  distraction = {
    by_anything = 3,
    by_damage = 4,
    by_enemy = 1,
    none = 0
  },
  entity_status = {
    cant_divide_segments = 10,
    charging = 6,
    closed_by_circuit_network = 40,
    disabled = 43,
    disabled_by_control_behavior = 39,
    disabled_by_script = 42,
    discharging = 7,
    fluid_ingredient_shortage = 20,
    full_output = 22,
    fully_charged = 8,
    item_ingredient_shortage = 21,
    launching_rocket = 30,
    low_input_fluid = 18,
    low_power = 12,
    low_temperature = 36,
    marked_for_deconstruction = 44,
    missing_required_fluid = 24,
    missing_science_packs = 25,
    networks_connected = 4,
    networks_disconnected = 5,
    no_ammo = 35,
    no_fuel = 37,
    no_ingredients = 14,
    no_input_fluid = 19,
    no_minable_resources = 17,
    no_modules_to_transmit = 31,
    no_power = 38,
    no_recipe = 15,
    no_research_in_progress = 16,
    normal = 2,
    not_connected_to_rail = 11,
    not_plugged_in_electric_network = 3,
    opened_by_circuit_network = 41,
    out_of_logistic_network = 13,
    preparing_rocket_for_launch = 28,
    recharging_after_power_outage = 32,
    turned_off_during_daytime = 9,
    waiting_for_source_items = 26,
    waiting_for_space_in_destination = 27,
    waiting_for_target_to_be_built = 33,
    waiting_for_train = 34,
    waiting_to_launch_rocket = 29,
    working = 1
  },
  events = {
    on_ai_command_completed = 120,
    on_area_cloned = 125,
    on_biter_base_built = 57,
    on_brush_cloned = 126,
    on_build_base_arrived = 143,
    on_built_entity = 6,
    on_cancelled_deconstruction = 23,
    on_cancelled_upgrade = 122,
    on_character_corpse_expired = 95,
    on_chart_tag_added = 145,
    on_chart_tag_modified = 146,
    on_chart_tag_removed = 147,
    on_chunk_charted = 104,
    on_chunk_deleted = 113,
    on_chunk_generated = 12,
    on_combat_robot_expired = 84,
    on_console_chat = 73,
    on_console_command = 74,
    on_cutscene_cancelled = 169,
    on_cutscene_finished = 182,
    on_cutscene_started = 181,
    on_cutscene_waypoint_reached = 35,
    on_difficulty_settings_changed = 62,
    on_entity_cloned = 124,
    on_entity_color_changed = 180,
    on_entity_damaged = 103,
    on_entity_destroyed = 160,
    on_entity_died = 4,
    on_entity_logistic_slot_changed = 173,
    on_entity_renamed = 59,
    on_entity_settings_pasted = 33,
    on_entity_spawned = 137,
    on_equipment_inserted = 174,
    on_equipment_removed = 175,
    on_force_cease_fire_changed = 153,
    on_force_created = 29,
    on_force_friends_changed = 152,
    on_force_reset = 106,
    on_forces_merged = 108,
    on_forces_merging = 30,
    on_game_created_from_scenario = 127,
    on_gui_checked_state_changed = 3,
    on_gui_click = 1,
    on_gui_closed = 89,
    on_gui_confirmed = 148,
    on_gui_elem_changed = 69,
    on_gui_hover = 178,
    on_gui_leave = 179,
    on_gui_location_changed = 149,
    on_gui_opened = 88,
    on_gui_selected_tab_changed = 150,
    on_gui_selection_state_changed = 60,
    on_gui_switch_state_changed = 151,
    on_gui_text_changed = 2,
    on_gui_value_changed = 90,
    on_land_mine_armed = 107,
    on_lua_shortcut = 36,
    on_marked_for_deconstruction = 22,
    on_marked_for_upgrade = 121,
    on_market_item_purchased = 55,
    on_mod_item_opened = 87,
    on_permission_group_added = 168,
    on_permission_group_deleted = 167,
    on_permission_group_edited = 163,
    on_permission_string_imported = 165,
    on_picked_up_item = 5,
    on_player_alt_reverse_selected_area = 177,
    on_player_alt_selected_area = 52,
    on_player_ammo_inventory_changed = 38,
    on_player_armor_inventory_changed = 37,
    on_player_banned = 115,
    on_player_built_tile = 47,
    on_player_cancelled_crafting = 102,
    on_player_changed_force = 58,
    on_player_changed_position = 86,
    on_player_changed_surface = 53,
    on_player_cheat_mode_disabled = 94,
    on_player_cheat_mode_enabled = 93,
    on_player_clicked_gps_tag = 161,
    on_player_configured_blueprint = 72,
    on_player_configured_spider_remote = 170,
    on_player_crafted_item = 13,
    on_player_created = 26,
    on_player_cursor_stack_changed = 31,
    on_player_deconstructed_area = 71,
    on_player_demoted = 78,
    on_player_died = 43,
    on_player_display_resolution_changed = 99,
    on_player_display_scale_changed = 100,
    on_player_driving_changed_state = 28,
    on_player_dropped_item = 56,
    on_player_fast_transferred = 132,
    on_player_flushed_fluid = 162,
    on_player_gun_inventory_changed = 39,
    on_player_input_method_changed = 183,
    on_player_joined_game = 45,
    on_player_kicked = 116,
    on_player_left_game = 46,
    on_player_main_inventory_changed = 34,
    on_player_mined_entity = 67,
    on_player_mined_item = 8,
    on_player_mined_tile = 48,
    on_player_muted = 91,
    on_player_pipette = 98,
    on_player_placed_equipment = 40,
    on_player_promoted = 77,
    on_player_removed = 75,
    on_player_removed_equipment = 41,
    on_player_repaired_entity = 131,
    on_player_respawned = 44,
    on_player_reverse_selected_area = 176,
    on_player_rotated_entity = 21,
    on_player_selected_area = 51,
    on_player_set_quick_bar_slot = 156,
    on_player_setup_blueprint = 70,
    on_player_toggled_alt_mode = 130,
    on_player_toggled_map_editor = 123,
    on_player_trash_inventory_changed = 109,
    on_player_unbanned = 117,
    on_player_unmuted = 92,
    on_player_used_capsule = 76,
    on_player_used_spider_remote = 171,
    on_post_entity_died = 136,
    on_pre_build = 9,
    on_pre_chunk_deleted = 133,
    on_pre_entity_settings_pasted = 32,
    on_pre_ghost_deconstructed = 96,
    on_pre_ghost_upgraded = 97,
    on_pre_permission_group_deleted = 166,
    on_pre_permission_string_imported = 164,
    on_pre_player_crafted_item = 101,
    on_pre_player_died = 42,
    on_pre_player_left_game = 110,
    on_pre_player_mined_item = 11,
    on_pre_player_removed = 138,
    on_pre_player_toggled_map_editor = 157,
    on_pre_robot_exploded_cliff = 134,
    on_pre_script_inventory_resized = 158,
    on_pre_surface_cleared = 112,
    on_pre_surface_deleted = 65,
    on_research_cancelled = 20,
    on_research_finished = 18,
    on_research_reversed = 19,
    on_research_started = 17,
    on_resource_depleted = 27,
    on_robot_built_entity = 14,
    on_robot_built_tile = 49,
    on_robot_exploded_cliff = 135,
    on_robot_mined = 16,
    on_robot_mined_entity = 66,
    on_robot_mined_tile = 50,
    on_robot_pre_mined = 15,
    on_rocket_launch_ordered = 118,
    on_rocket_launched = 10,
    on_runtime_mod_setting_changed = 61,
    on_script_inventory_resized = 159,
    on_script_path_request_finished = 119,
    on_script_trigger_effect = 155,
    on_sector_scanned = 7,
    on_selected_entity_changed = 54,
    on_spider_command_completed = 172,
    on_string_translated = 154,
    on_surface_cleared = 111,
    on_surface_created = 63,
    on_surface_deleted = 64,
    on_surface_imported = 128,
    on_surface_renamed = 129,
    on_technology_effects_reset = 105,
    on_tick = 0,
    on_train_changed_state = 25,
    on_train_created = 68,
    on_train_schedule_changed = 114,
    on_trigger_created_entity = 24,
    on_trigger_fired_artillery = 144,
    on_unit_added_to_group = 140,
    on_unit_group_created = 139,
    on_unit_group_finished_gathering = 142,
    on_unit_removed_from_group = 141,
    on_worker_robot_expired = 85,
    script_raised_built = 79,
    script_raised_destroy = 80,
    script_raised_revive = 81,
    script_raised_set_tiles = 83,
    script_raised_teleported = 82
  },
  flow_precision_index = {
    fifty_hours = 5,
    five_seconds = 0,
    one_hour = 3,
    one_minute = 1,
    one_thousand_hours = 7,
    ten_hours = 4,
    ten_minutes = 2,
    two_hundred_fifty_hours = 6
  },
  game_controller_interaction = {
    always = 0,
    never = 2,
    normal = 1
  },
  group_state = {
    attacking_distraction = 2,
    attacking_target = 3,
    finished = 4,
    gathering = 0,
    moving = 1,
    pathfinding = 5,
    wander_in_group = 6
  },
  gui_type = {
    achievement = 8,
    blueprint_library = 9,
    bonus = 6,
    controller = 3,
    custom = 16,
    entity = 1,
    equipment = 10,
    item = 5,
    logistic = 11,
    none = 0,
    other_player = 12,
    permissions = 14,
    player_management = 18,
    production = 4,
    research = 2,
    script_inventory = 23,
    server_management = 17,
    tile = 19,
    trains = 7,
    tutorials = 15
  },
  input_action = {
    activate_copy = 45,
    activate_cut = 46,
    activate_paste = 47,
    add_permission_group = 227,
    add_train_station = 96,
    admin_action = 195,
    alt_reverse_select_area = 164,
    alt_select_area = 162,
    alt_select_blueprint_entities = 121,
    alternative_copy = 119,
    begin_mining = 2,
    begin_mining_terrain = 62,
    build = 60,
    build_rail = 159,
    build_terrain = 152,
    cancel_craft = 83,
    cancel_deconstruct = 144,
    cancel_new_blueprint = 18,
    cancel_research = 160,
    cancel_upgrade = 145,
    change_active_character_tab = 100,
    change_active_item_group_for_crafting = 98,
    change_active_item_group_for_filters = 99,
    change_active_quick_bar = 231,
    change_arithmetic_combinator_parameters = 146,
    change_decider_combinator_parameters = 147,
    change_entity_label = 158,
    change_item_description = 157,
    change_item_label = 156,
    change_multiplayer_config = 194,
    change_picking_state = 199,
    change_programmable_speaker_alert_parameters = 149,
    change_programmable_speaker_circuit_parameters = 150,
    change_programmable_speaker_parameters = 148,
    change_riding_state = 63,
    change_shooting_state = 77,
    change_train_stop_station = 97,
    change_train_wait_condition = 153,
    change_train_wait_condition_data = 154,
    clear_cursor = 12,
    connect_rolling_stock = 9,
    copy = 118,
    copy_entity_settings = 20,
    copy_opened_blueprint = 124,
    copy_opened_item = 23,
    craft = 75,
    cursor_split = 71,
    cursor_transfer = 70,
    custom_input = 155,
    cycle_blueprint_book_backwards = 33,
    cycle_blueprint_book_forwards = 32,
    deconstruct = 116,
    delete_blueprint_library = 43,
    delete_blueprint_record = 129,
    delete_custom_tag = 225,
    delete_permission_group = 226,
    destroy_item = 67,
    destroy_opened_item = 22,
    disconnect_rolling_stock = 10,
    drag_train_schedule = 180,
    drag_train_wait_condition = 181,
    drop_blueprint_record = 128,
    drop_item = 59,
    edit_blueprint_tool_preview = 137,
    edit_custom_tag = 172,
    edit_permission_group = 173,
    export_blueprint = 139,
    fast_entity_split = 215,
    fast_entity_transfer = 213,
    flush_opened_entity_fluid = 54,
    flush_opened_entity_specific_fluid = 198,
    go_to_train_station = 240,
    grab_blueprint_record = 127,
    gui_checked_state_changed = 102,
    gui_click = 92,
    gui_confirmed = 93,
    gui_elem_changed = 178,
    gui_hover = 248,
    gui_leave = 249,
    gui_location_changed = 107,
    gui_selected_tab_changed = 104,
    gui_selection_state_changed = 103,
    gui_switch_state_changed = 106,
    gui_text_changed = 101,
    gui_value_changed = 105,
    import_blueprint = 140,
    import_blueprint_string = 174,
    import_blueprints_filtered = 141,
    import_permissions_string = 175,
    inventory_split = 82,
    inventory_transfer = 73,
    launch_rocket = 15,
    lua_shortcut = 196,
    map_editor_action = 191,
    market_offer = 95,
    mod_settings_changed = 170,
    open_achievements_gui = 31,
    open_blueprint_library_gui = 57,
    open_blueprint_record = 126,
    open_bonus_gui = 29,
    open_character_gui = 7,
    open_current_vehicle_gui = 8,
    open_equipment = 69,
    open_gui = 5,
    open_item = 64,
    open_logistic_gui = 40,
    open_mod_item = 68,
    open_parent_of_opened_item = 65,
    open_production_gui = 16,
    open_technology_gui = 14,
    open_tips_and_tricks_gui = 56,
    open_train_gui = 221,
    open_train_station_gui = 238,
    open_trains_gui = 30,
    paste_entity_settings = 21,
    place_equipment = 108,
    quick_bar_pick_slot = 188,
    quick_bar_set_selected_page = 189,
    quick_bar_set_slot = 187,
    reassign_blueprint = 125,
    remove_cables = 138,
    remove_train_station = 239,
    reset_assembling_machine = 13,
    reset_item = 66,
    reverse_select_area = 163,
    rotate_entity = 214,
    select_area = 161,
    select_blueprint_entities = 120,
    select_entity_slot = 183,
    select_item = 182,
    select_mapper_slot = 185,
    select_next_valid_gun = 41,
    select_tile_slot = 184,
    send_spidertron = 111,
    set_auto_launch_rocket = 207,
    set_autosort_inventory = 204,
    set_behavior_mode = 212,
    set_car_weapons_control = 229,
    set_circuit_condition = 86,
    set_circuit_mode_of_operation = 91,
    set_controller_logistic_trash_filter_item = 166,
    set_deconstruction_item_tile_selection_mode = 224,
    set_deconstruction_item_trees_and_rocks_only = 223,
    set_entity_color = 222,
    set_entity_energy_property = 171,
    set_entity_logistic_trash_filter_item = 167,
    set_filter = 84,
    set_flat_controller_gui = 205,
    set_heat_interface_mode = 237,
    set_heat_interface_temperature = 236,
    set_infinity_container_filter_item = 168,
    set_infinity_container_remove_unfiltered_items = 228,
    set_infinity_pipe_filter = 169,
    set_inserter_max_stack_size = 220,
    set_inventory_bar = 113,
    set_linked_container_link_i_d = 247,
    set_logistic_filter_item = 89,
    set_logistic_filter_signal = 90,
    set_player_color = 243,
    set_recipe_notifications = 206,
    set_request_from_buffers = 230,
    set_research_finished_stops_game = 219,
    set_signal = 87,
    set_splitter_priority = 234,
    set_train_stopped = 216,
    set_trains_limit = 245,
    set_vehicle_automatic_targeting_parameters = 151,
    setup_assembling_machine = 78,
    setup_blueprint = 122,
    setup_single_blueprint_record = 123,
    smart_pipette = 80,
    spawn_item = 132,
    stack_split = 81,
    stack_transfer = 72,
    start_repair = 115,
    start_research = 88,
    start_walking = 61,
    stop_building_by_moving = 53,
    switch_connect_to_logistic_network = 211,
    switch_constant_combinator_state = 208,
    switch_inserter_filter_mode_state = 210,
    switch_power_switch_state = 209,
    switch_to_rename_stop_gui = 28,
    take_equipment = 109,
    toggle_deconstruction_item_entity_filter_mode = 38,
    toggle_deconstruction_item_tile_filter_mode = 39,
    toggle_driving = 4,
    toggle_enable_vehicle_logistics_while_moving = 37,
    toggle_entity_logistic_requests = 52,
    toggle_equipment_movement_bonus = 50,
    toggle_map_editor = 42,
    toggle_personal_logistic_requests = 51,
    toggle_personal_roboport = 49,
    toggle_show_entity_info = 24,
    translate_string = 197,
    undo = 48,
    upgrade = 117,
    upgrade_opened_blueprint_by_item = 131,
    upgrade_opened_blueprint_by_record = 130,
    use_artillery_remote = 112,
    use_item = 110,
    wire_dragging = 76,
    write_to_console = 94
  },
  input_method = {
    game_controller = 1,
    keyboard_and_mouse = 0
  },
  inventory = {
    artillery_turret_ammo = 1,
    artillery_wagon_ammo = 1,
    assembling_machine_input = 2,
    assembling_machine_modules = 4,
    assembling_machine_output = 3,
    beacon_modules = 1,
    burnt_result = 6,
    car_ammo = 3,
    car_trunk = 2,
    cargo_wagon = 1,
    character_ammo = 4,
    character_armor = 5,
    character_corpse = 1,
    character_guns = 3,
    character_main = 1,
    character_trash = 8,
    character_vehicle = 7,
    chest = 1,
    editor_ammo = 4,
    editor_armor = 5,
    editor_guns = 3,
    editor_main = 1,
    fuel = 1,
    furnace_modules = 4,
    furnace_result = 3,
    furnace_source = 2,
    god_main = 2,
    item_main = 1,
    lab_input = 2,
    lab_modules = 3,
    mining_drill_modules = 2,
    roboport_material = 2,
    roboport_robot = 1,
    robot_cargo = 1,
    robot_repair = 2,
    rocket = 1,
    rocket_silo_input = 2,
    rocket_silo_modules = 4,
    rocket_silo_output = 3,
    rocket_silo_result = 6,
    rocket_silo_rocket = 5,
    spider_ammo = 3,
    spider_trash = 4,
    spider_trunk = 2,
    turret_ammo = 1
  },
  logistic_member_index = {
    character_provider = 2,
    character_requester = 0,
    character_storage = 1,
    generic_on_off_behavior = 0,
    logistic_container = 0,
    vehicle_storage = 1
  },
  logistic_mode = {
    active_provider = 1,
    buffer = 5,
    none = 0,
    passive_provider = 4,
    requester = 3,
    storage = 2
  },
  mouse_button_type = {
    left = 2,
    middle = 8,
    none = 1,
    right = 4
  },
  print_skip = {
    if_redundant = 1,
    if_visible = 2,
    never = 0
  },
  print_sound = {
    always = 1,
    never = 0,
    use_player_settings = 2
  },
  prototypes = {
    achievement = {
      achievement = 0,
      ["build-entity-achievement"] = 0,
      ["combat-robot-count"] = 0,
      ["construct-with-robots-achievement"] = 0,
      ["deconstruct-with-robots-achievement"] = 0,
      ["deliver-by-robots-achievement"] = 0,
      ["dont-build-entity-achievement"] = 0,
      ["dont-craft-manually-achievement"] = 0,
      ["dont-use-entity-in-energy-production-achievement"] = 0,
      ["finish-the-game-achievement"] = 0,
      ["group-attack-achievement"] = 0,
      ["kill-achievement"] = 0,
      ["player-damaged-achievement"] = 0,
      ["produce-achievement"] = 0,
      ["produce-per-hour-achievement"] = 0,
      ["research-achievement"] = 0,
      ["train-path-achievement"] = 0
    },
    ["ambient-sound"] = {
      ["ambient-sound"] = 0
    },
    ["ammo-category"] = {
      ["ammo-category"] = 0
    },
    animation = {
      animation = 0
    },
    ["autoplace-control"] = {
      ["autoplace-control"] = 0
    },
    ["custom-input"] = {
      ["custom-input"] = 0
    },
    ["damage-type"] = {
      ["damage-type"] = 0
    },
    decorative = {
      ["optimized-decorative"] = 0
    },
    ["editor-controller"] = {
      ["editor-controller"] = 0
    },
    entity = {
      accumulator = 0,
      ["ammo-turret"] = 0,
      ["arithmetic-combinator"] = 0,
      arrow = 0,
      ["artillery-flare"] = 0,
      ["artillery-projectile"] = 0,
      ["artillery-turret"] = 0,
      ["artillery-wagon"] = 0,
      ["assembling-machine"] = 0,
      beacon = 0,
      beam = 0,
      boiler = 0,
      ["burner-generator"] = 0,
      car = 0,
      ["cargo-wagon"] = 0,
      character = 0,
      ["character-corpse"] = 0,
      cliff = 0,
      ["combat-robot"] = 0,
      ["constant-combinator"] = 0,
      ["construction-robot"] = 0,
      container = 0,
      corpse = 0,
      ["curved-rail"] = 0,
      ["decider-combinator"] = 0,
      ["deconstructible-tile-proxy"] = 0,
      ["electric-energy-interface"] = 0,
      ["electric-pole"] = 0,
      ["electric-turret"] = 0,
      ["entity-ghost"] = 0,
      explosion = 0,
      fire = 0,
      fish = 0,
      ["flame-thrower-explosion"] = 0,
      ["fluid-turret"] = 0,
      ["fluid-wagon"] = 0,
      ["flying-text"] = 0,
      furnace = 0,
      gate = 0,
      generator = 0,
      ["heat-interface"] = 0,
      ["heat-pipe"] = 0,
      ["highlight-box"] = 0,
      ["infinity-container"] = 0,
      ["infinity-pipe"] = 0,
      inserter = 0,
      ["item-entity"] = 0,
      ["item-request-proxy"] = 0,
      lab = 0,
      lamp = 0,
      ["land-mine"] = 0,
      ["leaf-particle"] = 0,
      ["linked-belt"] = 0,
      ["linked-container"] = 0,
      loader = 0,
      ["loader-1x1"] = 0,
      locomotive = 0,
      ["logistic-container"] = 0,
      ["logistic-robot"] = 0,
      market = 0,
      ["mining-drill"] = 0,
      ["offshore-pump"] = 0,
      particle = 0,
      ["particle-source"] = 0,
      pipe = 0,
      ["pipe-to-ground"] = 0,
      ["player-port"] = 0,
      ["power-switch"] = 0,
      ["programmable-speaker"] = 0,
      projectile = 0,
      pump = 0,
      radar = 0,
      ["rail-chain-signal"] = 0,
      ["rail-remnants"] = 0,
      ["rail-signal"] = 0,
      reactor = 0,
      resource = 0,
      roboport = 0,
      ["rocket-silo"] = 0,
      ["rocket-silo-rocket"] = 0,
      ["rocket-silo-rocket-shadow"] = 0,
      ["simple-entity"] = 0,
      ["simple-entity-with-force"] = 0,
      ["simple-entity-with-owner"] = 0,
      smoke = 0,
      ["smoke-with-trigger"] = 0,
      ["solar-panel"] = 0,
      ["speech-bubble"] = 0,
      ["spider-leg"] = 0,
      ["spider-vehicle"] = 0,
      splitter = 0,
      sticker = 0,
      ["storage-tank"] = 0,
      ["straight-rail"] = 0,
      stream = 0,
      ["tile-ghost"] = 0,
      ["train-stop"] = 0,
      ["transport-belt"] = 0,
      tree = 0,
      turret = 0,
      ["underground-belt"] = 0,
      unit = 0,
      ["unit-spawner"] = 0,
      wall = 0
    },
    equipment = {
      ["active-defense-equipment"] = 0,
      ["battery-equipment"] = 0,
      ["belt-immunity-equipment"] = 0,
      ["energy-shield-equipment"] = 0,
      ["generator-equipment"] = 0,
      ["movement-bonus-equipment"] = 0,
      ["night-vision-equipment"] = 0,
      ["roboport-equipment"] = 0,
      ["solar-panel-equipment"] = 0
    },
    ["equipment-category"] = {
      ["equipment-category"] = 0
    },
    ["equipment-grid"] = {
      ["equipment-grid"] = 0
    },
    fluid = {
      fluid = 0
    },
    font = {
      font = 0
    },
    ["fuel-category"] = {
      ["fuel-category"] = 0
    },
    ["god-controller"] = {
      ["god-controller"] = 0
    },
    ["gui-style"] = {
      ["gui-style"] = 0
    },
    item = {
      ammo = 0,
      armor = 0,
      blueprint = 0,
      ["blueprint-book"] = 0,
      capsule = 0,
      ["copy-paste-tool"] = 0,
      ["deconstruction-item"] = 0,
      gun = 0,
      item = 0,
      ["item-with-entity-data"] = 0,
      ["item-with-inventory"] = 0,
      ["item-with-label"] = 0,
      ["item-with-tags"] = 0,
      ["mining-tool"] = 0,
      module = 0,
      ["rail-planner"] = 0,
      ["repair-tool"] = 0,
      ["selection-tool"] = 0,
      ["spidertron-remote"] = 0,
      tool = 0,
      ["upgrade-item"] = 0
    },
    ["item-group"] = {
      ["item-group"] = 0
    },
    ["item-subgroup"] = {
      ["item-subgroup"] = 0
    },
    ["map-gen-presets"] = {
      ["map-gen-presets"] = 0
    },
    ["map-settings"] = {
      ["map-settings"] = 0
    },
    ["module-category"] = {
      ["module-category"] = 0
    },
    ["mouse-cursor"] = {
      ["mouse-cursor"] = 0
    },
    ["noise-expression"] = {
      ["noise-expression"] = 0
    },
    ["noise-layer"] = {
      ["noise-layer"] = 0
    },
    particle = {
      ["optimized-particle"] = 0
    },
    recipe = {
      recipe = 0
    },
    ["recipe-category"] = {
      ["recipe-category"] = 0
    },
    ["resource-category"] = {
      ["resource-category"] = 0
    },
    shortcut = {
      shortcut = 0
    },
    sound = {
      sound = 0
    },
    ["spectator-controller"] = {
      ["spectator-controller"] = 0
    },
    sprite = {
      sprite = 0
    },
    technology = {
      technology = 0
    },
    tile = {
      tile = 0
    },
    ["tile-effect"] = {
      ["tile-effect"] = 0
    },
    ["tips-and-tricks-item"] = {
      ["tips-and-tricks-item"] = 0
    },
    ["tips-and-tricks-item-category"] = {
      ["tips-and-tricks-item-category"] = 0
    },
    ["trigger-target-type"] = {
      ["trigger-target-type"] = 0
    },
    ["trivial-smoke"] = {
      ["trivial-smoke"] = 0
    },
    tutorial = {
      tutorial = 0
    },
    ["utility-constants"] = {
      ["utility-constants"] = 0
    },
    ["utility-sounds"] = {
      ["utility-sounds"] = 0
    },
    ["utility-sprites"] = {
      ["utility-sprites"] = 0
    },
    ["virtual-signal"] = {
      ["virtual-signal"] = 0
    },
    ["wind-sound"] = {
      ["wind-sound"] = 0
    }
  },
  rail_connection_direction = {
    left = 0,
    none = 3,
    right = 2,
    straight = 1
  },
  rail_direction = {
    back = 1,
    front = 0
  },
  relative_gui_position = {
    bottom = 1,
    left = 2,
    right = 3,
    top = 0
  },
  relative_gui_type = {
    accumulator_gui = "accumulator-gui",
    achievement_gui = "achievement-gui",
    additional_entity_info_gui = "additional-entity-info-gui",
    admin_gui = "admin-gui",
    arithmetic_combinator_gui = "arithmetic-combinator-gui",
    armor_gui = "armor-gui",
    assembling_machine_gui = "assembling-machine-gui",
    assembling_machine_select_recipe_gui = "assembling-machine-select-recipe-gui",
    beacon_gui = "beacon-gui",
    blueprint_book_gui = "blueprint-book-gui",
    blueprint_library_gui = "blueprint-library-gui",
    blueprint_setup_gui = "blueprint-setup-gui",
    bonus_gui = "bonus-gui",
    burner_equipment_gui = "burner-equipment-gui",
    car_gui = "car-gui",
    constant_combinator_gui = "constant-combinator-gui",
    container_gui = "container-gui",
    controller_gui = "controller-gui",
    decider_combinator_gui = "decider-combinator-gui",
    deconstruction_item_gui = "deconstruction-item-gui",
    electric_energy_interface_gui = "electric-energy-interface-gui",
    electric_network_gui = "electric-network-gui",
    entity_variations_gui = "entity-variations-gui",
    entity_with_energy_source_gui = "entity-with-energy-source-gui",
    equipment_grid_gui = "equipment-grid-gui",
    furnace_gui = "furnace-gui",
    generic_on_off_entity_gui = "generic-on-off-entity-gui",
    heat_interface_gui = "heat-interface-gui",
    infinity_pipe_gui = "infinity-pipe-gui",
    inserter_gui = "inserter-gui",
    item_with_inventory_gui = "item-with-inventory-gui",
    lab_gui = "lab-gui",
    lamp_gui = "lamp-gui",
    linked_container_gui = "linked-container-gui",
    loader_gui = "loader-gui",
    logistic_gui = "logistic-gui",
    market_gui = "market-gui",
    mining_drill_gui = "mining-drill-gui",
    other_player_gui = "other-player-gui",
    permissions_gui = "permissions-gui",
    pipe_gui = "pipe-gui",
    power_switch_gui = "power-switch-gui",
    production_gui = "production-gui",
    programmable_speaker_gui = "programmable-speaker-gui",
    rail_chain_signal_gui = "rail-chain-signal-gui",
    rail_signal_gui = "rail-signal-gui",
    reactor_gui = "reactor-gui",
    rename_stop_gui = "rename-stop-gui",
    resource_entity_gui = "resource-entity-gui",
    roboport_gui = "roboport-gui",
    rocket_silo_gui = "rocket-silo-gui",
    script_inventory_gui = "script-inventory-gui",
    server_config_gui = "server-config-gui",
    spider_vehicle_gui = "spider-vehicle-gui",
    splitter_gui = "splitter-gui",
    standalone_character_gui = "standalone-character-gui",
    storage_tank_gui = "storage-tank-gui",
    tile_variations_gui = "tile-variations-gui",
    train_gui = "train-gui",
    train_stop_gui = "train-stop-gui",
    trains_gui = "trains-gui",
    transport_belt_gui = "transport-belt-gui",
    upgrade_item_gui = "upgrade-item-gui",
    wall_gui = "wall-gui"
  },
  render_mode = {
    chart = 2,
    chart_zoomed_in = 3,
    game = 1
  },
  rich_text_setting = {
    disabled = 0,
    enabled = 17,
    highlight = 30
  },
  riding = {
    acceleration = {
      accelerating = 1,
      braking = 2,
      nothing = 0,
      reversing = 3
    },
    direction = {
      left = 0,
      right = 2,
      straight = 1
    }
  },
  rocket_silo_status = {
    arms_advance = 6,
    arms_retract = 10,
    building_rocket = 0,
    create_rocket = 1,
    doors_closing = 13,
    doors_opened = 4,
    doors_opening = 3,
    engine_starting = 9,
    launch_started = 14,
    launch_starting = 8,
    lights_blinking_close = 12,
    lights_blinking_open = 2,
    rocket_flying = 11,
    rocket_ready = 7,
    rocket_rising = 5
  },
  shooting = {
    not_shooting = 0,
    shooting_enemies = 1,
    shooting_selected = 2
  },
  signal_state = {
    closed = 1,
    open = 0,
    reserved = 2,
    reserved_by_circuit_network = 3
  },
  train_state = {
    arrive_signal = 4,
    arrive_station = 6,
    destination_full = 10,
    manual_control = 9,
    manual_control_stop = 8,
    no_path = 3,
    no_schedule = 2,
    on_the_path = 0,
    path_lost = 1,
    wait_signal = 5,
    wait_station = 7
  },
  transport_line = {
    left_line = 1,
    left_split_line = 5,
    left_underground_line = 3,
    right_line = 2,
    right_split_line = 6,
    right_underground_line = 4,
    secondary_left_line = 3,
    secondary_left_split_line = 7,
    secondary_right_line = 4,
    secondary_right_split_line = 8
  },
  wire_connection_id = {
    electric_pole = 0,
    power_switch_left = 0,
    power_switch_right = 1
  },
  wire_type = {
    copper = 1,
    green = 3,
    red = 2
  }
}