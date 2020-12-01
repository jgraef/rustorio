use serde::{Deserialize, Serialize};

use super::Prototype;
use crate::types::*; // TODO: Import only specific types

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FluidStream {
    /// particle_horizontal_speed :: double
    particle_horizontal_speed: f64,

    /// particle_horizontal_speed_deviation :: double
    particle_horizontal_speed_deviation: f64,

    /// particle_spawn_interval :: uint16
    particle_spawn_interval: u16,

    /// particle_vertical_acceleration :: double
    particle_vertical_acceleration: f64,

    /// action :: Trigger (optional)
    action: Option<Trigger>,

    /// ground_light :: LightDefinition (optional)
    ground_light: Option<LightDefinition>,

    /// initial_action :: Trigger (optional)
    initial_action: Option<Trigger>,

    /// oriented_particle :: bool (optional)
    oriented_particle: Option<bool>,

    /// particle :: Animation (optional)
    particle: Option<Animation>,

    /// particle_alpha_per_part :: float (optional)
    particle_alpha_per_part: Option<f32>,

    /// particle_buffer_size :: uint32 (optional)
    particle_buffer_size: Option<u32>,

    /// particle_end_alpha :: float (optional)
    particle_end_alpha: Option<f32>,

    /// particle_fade_out_duration :: uint16 (optional)
    particle_fade_out_duration: Option<u16>,

    /// particle_fade_out_threshold :: float (optional)
    particle_fade_out_threshold: Option<f32>,

    /// particle_loop_exit_threshold :: float (optional)
    particle_loop_exit_threshold: Option<f32>,

    /// particle_loop_frame_count :: uint16 (optional)
    particle_loop_frame_count: Option<u16>,

    /// particle_scale_per_part :: float (optional)
    particle_scale_per_part: Option<f32>,

    /// particle_spawn_timeout :: uint16 (optional)
    particle_spawn_timeout: Option<u16>,

    /// particle_start_alpha :: float (optional)
    particle_start_alpha: Option<f32>,

    /// particle_start_scale :: float (optional)
    particle_start_scale: Option<f32>,

    /// progress_to_create_smoke :: float (optional)
    progress_to_create_smoke: Option<f32>,

    /// shadow :: Animation (optional)
    shadow: Option<Animation>,

    /// shadow_scale_enabled :: bool (optional)
    shadow_scale_enabled: Option<bool>,

    /// smoke_sources :: table of SmokeSource (optional)
    smoke_sources: Option<Vec<SmokeSource>>,

    /// special_neutral_target_damage :: DamagePrototype (optional)
    special_neutral_target_damage: Option<DamagePrototype>,

    /// spine_animation :: Animation (optional)
    spine_animation: Option<Animation>,

    /// stream_light :: LightDefinition (optional)
    stream_light: Option<LightDefinition>,

    /// target_position_deviation :: double (optional)
    target_position_deviation: Option<f64>,

    /// width :: float (optional)
    width: Option<f32>,
}

impl Prototype for FluidStream {
    const TYPE: Option<&'static str> = Some("stream");
}
