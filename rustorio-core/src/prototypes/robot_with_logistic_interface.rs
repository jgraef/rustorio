use serde::{Deserialize, Serialize};

use super::Prototype;
use crate::types::*; // TODO: Import only specific types

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RobotWithLogisticInterface {
    /// cargo_centered :: vector
    cargo_centered: Vector2<f32>,

    /// max_payload_size :: ItemCountType
    max_payload_size: ItemCountType,

    /// draw_cargo :: bool (optional)
    draw_cargo: Option<bool>,

    /// idle :: RotatedAnimation (optional)
    idle: Option<RotatedAnimation>,

    /// in_motion :: RotatedAnimation (optional)
    in_motion: Option<RotatedAnimation>,

    /// shadow_idle :: RotatedAnimation (optional)
    shadow_idle: Option<RotatedAnimation>,

    /// shadow_in_motion :: RotatedAnimation (optional)
    shadow_in_motion: Option<RotatedAnimation>,
}

impl Prototype for RobotWithLogisticInterface {
    const TYPE: Option<&'static str> = Some("None");
}
