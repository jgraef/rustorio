use serde::{Deserialize, Serialize};

use super::Prototype;
use crate::types::*; // TODO: Import only specific types

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EnemySpawner {
    /// animations :: AnimationVariations
    animations: AnimationVariations,

    /// call_for_help_radius :: double
    call_for_help_radius: f64,

    /// max_count_of_owned_units :: uint32
    max_count_of_owned_units: u32,

    /// max_friends_around_to_spawn :: uint32
    max_friends_around_to_spawn: u32,

    /// max_richness_for_spawn_shift :: double
    max_richness_for_spawn_shift: f64,

    /// max_spawn_shift :: double
    max_spawn_shift: f64,

    /// pollution_absorption_absolute :: double
    pollution_absorption_absolute: f64,

    /// pollution_absorption_proportional :: double
    pollution_absorption_proportional: f64,

    /// result_units :: table of UnitSpawnDefinition
    result_units: Vec<UnitSpawnDefinition>,

    /// spawning_cooldown :: array of two double
    spawning_cooldown: [f64; 2],

    /// spawning_radius :: double
    spawning_radius: f64,

    /// spawning_spacing :: double
    spawning_spacing: f64,

    /// dying_sound :: Sound (optional)
    dying_sound: Option<Sound>,

    /// integration :: SpriteVariations (optional)
    integration: Option<SpriteVariations>,

    /// max_darkness_to_spawn :: float (optional)
    max_darkness_to_spawn: Option<f32>,

    /// min_darkness_to_spawn :: float (optional)
    min_darkness_to_spawn: Option<f32>,

    /// random_animation_offset :: bool (optional)
    random_animation_offset: Option<bool>,
}

impl Prototype for EnemySpawner {
    const TYPE: Option<&'static str> = Some("unit-spawner");
}
