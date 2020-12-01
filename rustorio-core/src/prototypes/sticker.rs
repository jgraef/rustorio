use serde::{Deserialize, Serialize};

use super::Prototype;
use crate::types::*; // TODO: Import only specific types

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Sticker {
    /// duration_in_ticks :: uint32
    duration_in_ticks: u32,

    /// animation :: Animation (optional)
    animation: Option<Animation>,

    /// damage_interval :: uint32 (optional)
    damage_interval: Option<u32>,

    /// damage_per_tick :: DamagePrototype (optional)
    damage_per_tick: Option<DamagePrototype>,

    /// fire_spread_cooldown :: uint8 (optional)
    fire_spread_cooldown: Option<u8>,

    /// fire_spread_radius :: float (optional)
    fire_spread_radius: Option<f32>,

    /// force_visibility :: ForceCondition (optional)
    force_visibility: Option<ForceCondition>,

    /// selection_box_type :: string (optional)
    selection_box_type: Option<String>,

    /// single_particle :: bool (optional)
    single_particle: Option<bool>,

    /// spread_fire_entity :: string (optional)
    spread_fire_entity: Option<String>,

    /// stickers_per_square_meter :: float (optional)
    stickers_per_square_meter: Option<f32>,

    /// target_movement_modifier :: float (optional)
    target_movement_modifier: Option<f32>,

    /// target_movement_modifier_from :: float (optional)
    target_movement_modifier_from: Option<f32>,

    /// target_movement_modifier_to :: float (optional)
    target_movement_modifier_to: Option<f32>,

    /// vehicle_friction_modifier :: float (optional)
    vehicle_friction_modifier: Option<f32>,

    /// vehicle_friction_modifier_from :: float (optional)
    vehicle_friction_modifier_from: Option<f32>,

    /// vehicle_friction_modifier_to :: float (optional)
    vehicle_friction_modifier_to: Option<f32>,

    /// vehicle_speed_modifier :: float (optional)
    vehicle_speed_modifier: Option<f32>,

    /// vehicle_speed_modifier_from :: float (optional)
    vehicle_speed_modifier_from: Option<f32>,

    /// vehicle_speed_modifier_to :: float (optional)
    vehicle_speed_modifier_to: Option<f32>,
}

impl Prototype for Sticker {
    const TYPE: Option<&'static str> = Some("sticker");
}
