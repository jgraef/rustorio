use serde::{Serialize, Deserialize};
        
use super::{Prototype, type_stubs::*};
        
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UtilitySounds {
    /// achievement_unlocked :: Sound
    achievement_unlocked: Sound,    

    /// alert_destroyed :: Sound
    alert_destroyed: Sound,    

    /// armor_insert :: Sound
    armor_insert: Sound,    

    /// armor_remove :: Sound
    armor_remove: Sound,    

    /// axe_fighting :: Sound
    axe_fighting: Sound,    

    /// axe_mining_ore :: Sound
    axe_mining_ore: Sound,    

    /// build_big :: Sound
    build_big: Sound,    

    /// build_medium :: Sound
    build_medium: Sound,    

    /// build_small :: Sound
    build_small: Sound,    

    /// cannot_build :: Sound
    cannot_build: Sound,    

    /// console_message :: Sound
    console_message: Sound,    

    /// crafting_finished :: Sound
    crafting_finished: Sound,    

    /// deconstruct_big :: Sound
    deconstruct_big: Sound,    

    /// deconstruct_medium :: Sound
    deconstruct_medium: Sound,    

    /// deconstruct_robot :: Sound
    deconstruct_robot: Sound,    

    /// deconstruct_small :: Sound
    deconstruct_small: Sound,    

    /// default_manual_repair :: Sound
    default_manual_repair: Sound,    

    /// game_lost :: Sound
    game_lost: Sound,    

    /// game_won :: Sound
    game_won: Sound,    

    /// gui_click :: Sound
    gui_click: Sound,    

    /// inventory_move :: Sound
    inventory_move: Sound,    

    /// list_box_click :: Sound
    list_box_click: Sound,    

    /// metal_walking_sound :: Sound
    metal_walking_sound: Sound,    

    /// mining_wood :: Sound
    mining_wood: Sound,    

    /// new_objective :: Sound
    new_objective: Sound,    

    /// picked_up_item :: Sound
    picked_up_item: Sound,    

    /// research_completed :: Sound
    research_completed: Sound,    

    /// rotated_big :: Sound
    rotated_big: Sound,    

    /// rotated_medium :: Sound
    rotated_medium: Sound,    

    /// rotated_small :: Sound
    rotated_small: Sound,    

    /// scenario_message :: Sound
    scenario_message: Sound,    

    /// smart_pipette :: Sound
    smart_pipette: Sound,    

    /// switch_gun :: Sound
    switch_gun: Sound,    

    /// tutorial_notice :: Sound
    tutorial_notice: Sound,    

    /// wire_connect_pole :: Sound
    wire_connect_pole: Sound,    

    /// wire_disconnect :: Sound
    wire_disconnect: Sound,    

    /// wire_pickup :: Sound
    wire_pickup: Sound,    

}

impl Prototype for UtilitySounds {
    const TYPE: Option<&'static str> = Some("utility-sounds");
}


