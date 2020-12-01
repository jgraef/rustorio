use serde::{Deserialize, Serialize};

use super::Prototype;
use crate::types::*; // TODO: Import only specific types

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FireFlame {
    /// damage_per_tick :: DamagePrototype
    damage_per_tick: DamagePrototype,

    /// spread_delay :: uint32
    spread_delay: u32,

    /// spread_delay_deviation :: uint32
    spread_delay_deviation: u32,

    /// add_fuel_cooldown :: uint32 (optional)
    add_fuel_cooldown: Option<u32>,

    /// burnt_patch_alpha_default :: float (optional)
    burnt_patch_alpha_default: Option<f32>,

    /// burnt_patch_alpha_variations :: table (optional)
    burnt_patch_alpha_variations: Option<Vec<Todo>>,

    /// burnt_patch_lifetime :: uint32 (optional)
    burnt_patch_lifetime: Option<u32>,

    /// burnt_patch_pictures :: SpriteVariations (optional)
    burnt_patch_pictures: Option<SpriteVariations>,

    /// damage_multiplier_decrease_per_tick :: float (optional)
    damage_multiplier_decrease_per_tick: Option<f32>,

    /// damage_multiplier_increase_per_added_fuel :: float (optional)
    damage_multiplier_increase_per_added_fuel: Option<f32>,

    /// delay_between_initial_flames :: uint32 (optional)
    delay_between_initial_flames: Option<u32>,

    /// fade_in_duration :: uint32 (optional)
    fade_in_duration: Option<u32>,

    /// fade_out_duration :: uint32 (optional)
    fade_out_duration: Option<u32>,

    /// flame_alpha :: float (optional)
    flame_alpha: Option<f32>,

    /// flame_alpha_deviation :: float (optional)
    flame_alpha_deviation: Option<f32>,

    /// initial_flame_count :: uint8 (optional)
    initial_flame_count: Option<u8>,

    /// initial_lifetime :: uint32 (optional)
    initial_lifetime: Option<u32>,

    /// initial_render_layer :: RenderLayer (optional)
    initial_render_layer: Option<RenderLayer>,

    /// lifetime_increase_by :: uint32 (optional)
    lifetime_increase_by: Option<u32>,

    /// lifetime_increase_cooldown :: uint32 (optional)
    lifetime_increase_cooldown: Option<u32>,

    /// light :: LightDefinition (optional)
    light: Option<LightDefinition>,

    /// limit_overlapping_particles :: bool (optional)
    limit_overlapping_particles: Option<bool>,

    /// maximum_damage_multiplier :: float (optional)
    maximum_damage_multiplier: Option<f32>,

    /// maximum_lifetime :: uint32 (optional)
    maximum_lifetime: Option<u32>,

    /// maximum_spread_count :: uint16 (optional)
    maximum_spread_count: Option<u16>,

    /// on_damage_tick_effect :: Trigger (optional)
    on_damage_tick_effect: Option<Trigger>,

    /// on_fuel_added_action :: Trigger (optional)
    on_fuel_added_action: Option<Trigger>,

    /// particle_alpha :: float (optional)
    particle_alpha: Option<f32>,

    /// particle_alpha_blend_duration :: uint16 (optional)
    particle_alpha_blend_duration: Option<u16>,

    /// particle_alpha_deviation :: float (optional)
    particle_alpha_deviation: Option<f32>,

    /// pictures :: AnimationVariations (optional)
    pictures: Option<AnimationVariations>,

    /// render_layer :: RenderLayer (optional)
    render_layer: Option<RenderLayer>,

    /// secondary_picture_fade_out_duration :: uint32 (optional)
    secondary_picture_fade_out_duration: Option<u32>,

    /// secondary_picture_fade_out_start :: uint32 (optional)
    secondary_picture_fade_out_start: Option<u32>,

    /// secondary_pictures :: AnimationVariations (optional)
    secondary_pictures: Option<AnimationVariations>,

    /// secondary_render_layer :: RenderLayer (optional)
    secondary_render_layer: Option<RenderLayer>,

    /// small_tree_fire_pictures :: AnimationVariations (optional)
    small_tree_fire_pictures: Option<AnimationVariations>,

    /// smoke :: Array of SmokeSource (optional)
    smoke: Option<Vec<SmokeSource>>,

    /// smoke_fade_in_duration :: uint32 (optional)
    smoke_fade_in_duration: Option<u32>,

    /// smoke_fade_out_duration :: uint32 (optional)
    smoke_fade_out_duration: Option<u32>,

    /// smoke_source_pictures :: AnimationVariations (optional)
    smoke_source_pictures: Option<AnimationVariations>,

    /// spawn_entity :: string (optional)
    spawn_entity: Option<String>,

    /// tree_dying_factor :: float (optional)
    tree_dying_factor: Option<f32>,

    /// uses_alternative_behavior :: bool (optional)
    uses_alternative_behavior: Option<bool>,
}

impl Prototype for FireFlame {
    const TYPE: Option<&'static str> = Some("fire");
}
