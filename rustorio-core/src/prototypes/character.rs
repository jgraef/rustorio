use serde::{Deserialize, Serialize};

use super::Prototype;
use crate::types::*; // TODO: Import only specific types

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Character {
    /// animations :: table of CharacterArmorAnimation
    animations: Vec<CharacterArmorAnimation>,

    /// build_distance :: uint32
    build_distance: u32,

    /// damage_hit_tint :: Color
    damage_hit_tint: Color,

    /// distance_per_frame :: double
    distance_per_frame: f64,

    /// drop_item_distance :: uint32
    drop_item_distance: u32,

    /// eat :: Sound
    eat: Sound,

    /// heartbeat :: Sound
    heartbeat: Sound,

    /// inventory_size :: ItemStackIndex
    inventory_size: ItemStackIndex,

    /// item_pickup_distance :: double
    item_pickup_distance: f64,

    /// loot_pickup_distance :: double
    loot_pickup_distance: f64,

    /// maximum_corner_sliding_distance :: double
    maximum_corner_sliding_distance: f64,

    /// mining_speed :: double
    mining_speed: f64,

    /// mining_with_tool_particles_animation_positions :: table of float
    mining_with_tool_particles_animation_positions: Vec<f32>,

    /// reach_distance :: uint32
    reach_distance: u32,

    /// reach_resource_distance :: double
    reach_resource_distance: f64,

    /// running_sound_animation_positions :: table of float
    running_sound_animation_positions: Vec<f32>,

    /// running_speed :: double
    running_speed: f64,

    /// ticks_to_keep_aiming_direction :: uint32
    ticks_to_keep_aiming_direction: u32,

    /// ticks_to_keep_gun :: uint32
    ticks_to_keep_gun: u32,

    /// ticks_to_stay_in_combat :: uint32
    ticks_to_stay_in_combat: u32,

    /// character_corpse :: string (optional)
    character_corpse: Option<String>,

    /// crafting_categories :: table of string (optional)
    crafting_categories: Option<Vec<String>>,

    /// enter_vehicle_distance :: double (optional)
    enter_vehicle_distance: Option<f64>,

    /// footprint_particles :: table of FootprintParticle (optional)
    footprint_particles: Option<Vec<FootprintParticle>>,

    /// footstep_particle_triggers :: FootstepTriggerEffectList (optional)
    footstep_particle_triggers: Option<FootstepTriggerEffectList>,

    /// has_belt_immunity :: bool (optional)
    has_belt_immunity: Option<bool>,

    /// left_footprint_frames :: table of float (optional)
    left_footprint_frames: Option<Vec<f32>>,

    /// left_footprint_offset :: vector (optional)
    left_footprint_offset: Option<Vector2<f32>>,

    /// light :: LightDefinition (optional)
    light: Option<LightDefinition>,

    /// mining_categories :: table of string (optional)
    mining_categories: Option<Vec<String>>,

    /// respawn_time :: uint32 (optional)
    respawn_time: Option<u32>,

    /// right_footprint_frames :: table of float (optional)
    right_footprint_frames: Option<Vec<f32>>,

    /// right_footprint_offset :: vector (optional)
    right_footprint_offset: Option<Vector2<f32>>,

    /// synced_footstep_particle_triggers :: FootstepTriggerEffectList (optional)
    synced_footstep_particle_triggers: Option<FootstepTriggerEffectList>,

    /// tool_attack_distance :: double (optional)
    tool_attack_distance: Option<f64>,

    /// tool_attack_result :: Trigger (optional)
    tool_attack_result: Option<Trigger>,
}

impl Prototype for Character {
    const TYPE: Option<&'static str> = Some("character");
}
