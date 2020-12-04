use serde::{Deserialize, Serialize};

use crate::prototypes::{Prototype, Visitor};
use crate::types::*; // TODO: Import only specific types

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LogisticRobot {
    /// idle_with_cargo :: RotatedAnimation (optional)
    idle_with_cargo: Option<RotatedAnimation>,

    /// in_motion_with_cargo :: RotatedAnimation (optional)
    in_motion_with_cargo: Option<RotatedAnimation>,

    /// shadow_idle_with_cargo :: RotatedAnimation (optional)
    shadow_idle_with_cargo: Option<RotatedAnimation>,

    /// shadow_in_motion_with_cargo :: RotatedAnimation (optional)
    shadow_in_motion_with_cargo: Option<RotatedAnimation>,
}

impl Prototype for LogisticRobot {
    const TYPE: Option<&'static str> = Some("logistic-robot");
}
