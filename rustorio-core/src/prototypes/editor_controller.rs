use serde::{Serialize, Deserialize};
        
use super::{Prototype, type_stubs::*};
        
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EditorController {
    /// adjust_speed_based_off_zoom :: bool
    adjust_speed_based_off_zoom: bool,    

    /// enable_flash_light :: bool
    enable_flash_light: bool,    

    /// fill_built_entity_energy_buffers :: bool
    fill_built_entity_energy_buffers: bool,    

    /// generate_neighbor_chunks :: bool
    generate_neighbor_chunks: bool,    

    /// gun_inventory_size :: ItemStackIndex
    gun_inventory_size: ItemStackIndex,    

    /// instant_blueprint_building :: bool
    instant_blueprint_building: bool,    

    /// instant_deconstruction :: bool
    instant_deconstruction: bool,    

    /// instant_rail_planner :: bool
    instant_rail_planner: bool,    

    /// instant_upgrading :: bool
    instant_upgrading: bool,    

    /// inventory_size :: ItemStackIndex
    inventory_size: ItemStackIndex,    

    /// item_pickup_distance :: double
    item_pickup_distance: f64,    

    /// loot_pickup_distance :: double
    loot_pickup_distance: f64,    

    /// mining_speed :: double
    mining_speed: f64,    

    /// movement_speed :: double
    movement_speed: f64,    

    /// name :: string
    name: String,    

    /// placed_corpses_never_expire :: bool
    placed_corpses_never_expire: bool,    

    /// render_as_day :: bool
    render_as_day: bool,    

    /// show_additional_entity_info_gui :: bool
    show_additional_entity_info_gui: bool,    

    /// show_character_tab_in_controller_gui :: bool
    show_character_tab_in_controller_gui: bool,    

    /// show_entity_health_bars :: bool
    show_entity_health_bars: bool,    

    /// show_entity_tags :: bool
    show_entity_tags: bool,    

    /// show_hidden_entities :: bool
    show_hidden_entities: bool,    

    /// show_infinity_filters_in_controller_gui :: bool
    show_infinity_filters_in_controller_gui: bool,    

    /// show_status_icons :: bool
    show_status_icons: bool,    

    /// type :: string
    r#type: String,    

}

impl Prototype for EditorController {
    const TYPE: Option<&'static str> = Some("editor-controller");
}


