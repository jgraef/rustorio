use serde::{Deserialize, Serialize};

use crate::types::*; // TODO: Import only specific types

use rustorio_data_derive::{FromLuaTable, Prototype};


#[derive(Clone, Debug, Serialize, Deserialize, FromLuaTable, Prototype)]
#[prototype(inherits="super::rolling_stock::RollingStock")]
pub struct ArtilleryWagon {
    /// ammo_stack_limit :: ItemCountType
    ammo_stack_limit: ItemCountType,

    /// gun :: string
    gun: String,

    /// inventory_size :: ItemStackIndex
    inventory_size: ItemStackIndex,

    /// manual_range_modifier :: double
    manual_range_modifier: f64,

    /// turret_rotation_speed :: double
    turret_rotation_speed: f64,

    /// cannon_barrel_light_direction :: Vector3<f32> (optional)
    cannon_barrel_light_direction: Option<Vector3<f32>>,

    /// cannon_barrel_pictures :: RotatedSprite (optional)
    cannon_barrel_pictures: Option<RotatedSprite>,

    /// cannon_barrel_recoil_shiftings :: table of Vector3D (optional)
    cannon_barrel_recoil_shiftings: Option<Vec<Vector3<f32>>>,

    /// cannon_barrel_recoil_shiftings_load_correction_matrix :: table of Vector3D (optional)
    cannon_barrel_recoil_shiftings_load_correction_matrix: Option<Vec<Vector3<f32>>>,

    /// cannon_base_pictures :: RotatedSprite (optional)
    cannon_base_pictures: Option<RotatedSprite>,

    /// cannon_base_shiftings :: table of vector (optional)
    cannon_base_shiftings: Option<Vec<Vector2<f32>>>,

    /// cannon_parking_frame_count :: uint16 (optional)
    cannon_parking_frame_count: Option<u16>,

    /// cannon_parking_speed :: float (optional)
    cannon_parking_speed: Option<f32>,

    /// disable_automatic_firing :: bool (optional)
    disable_automatic_firing: Option<bool>,

    /// rotating_sound :: InterruptibleSound (optional)
    rotating_sound: Option<InterruptibleSound>,

    /// rotating_stopped_sound :: Sound (optional)
    rotating_stopped_sound: Option<Sound>,

    /// turn_after_shooting_cooldown :: uint16 (optional)
    turn_after_shooting_cooldown: Option<u16>,
}
