use serde::{Deserialize, Serialize};

use super::Prototype;
use crate::types::*; // TODO: Import only specific types

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LogisticContainer {
    /// logistic_mode :: string
    logistic_mode: String,

    /// animation :: Animation (optional)
    animation: Option<Animation>,

    /// animation_sound :: Sound (optional)
    animation_sound: Option<Sound>,

    /// landing_location_offset :: vector (optional)
    landing_location_offset: Option<Vector2<f32>>,

    /// logistic_slots_count :: uint32 (optional)
    logistic_slots_count: Option<u32>,

    /// opened_duration :: uint8 (optional)
    opened_duration: Option<u8>,

    /// render_not_in_network_icon :: bool (optional)
    render_not_in_network_icon: Option<bool>,
}

impl Prototype for LogisticContainer {
    const TYPE: Option<&'static str> = Some("logistic-container");
}
