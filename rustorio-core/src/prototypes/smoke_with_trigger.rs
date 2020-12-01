use serde::{Deserialize, Serialize};

use super::Prototype;
use crate::types::*; // TODO: Import only specific types

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SmokeWithTrigger {
    /// action :: Trigger (optional)
    action: Option<Trigger>,

    /// action_cooldown :: uint32 (optional)
    action_cooldown: Option<u32>,

    /// particle_count :: uint8 (optional)
    particle_count: Option<u8>,

    /// particle_distance_scale_factor :: float (optional)
    particle_distance_scale_factor: Option<f32>,

    /// particle_duration_variation :: uint32 (optional)
    particle_duration_variation: Option<u32>,

    /// particle_scale_factor :: vector (optional)
    particle_scale_factor: Option<Vector2<f32>>,

    /// particle_spread :: vector (optional)
    particle_spread: Option<Vector2<f32>>,

    /// spread_duration_variation :: uint32 (optional)
    spread_duration_variation: Option<u32>,

    /// wave_distance :: vector (optional)
    wave_distance: Option<Vector2<f32>>,

    /// wave_speed :: vector (optional)
    wave_speed: Option<Vector2<f32>>,
}

impl Prototype for SmokeWithTrigger {
    const TYPE: Option<&'static str> = Some("smoke-with-trigger");
}
