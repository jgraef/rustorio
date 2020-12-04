use serde::{Deserialize, Serialize};

use crate::prototypes::{Prototype, Visitor};
use crate::types::*; // TODO: Import only specific types

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RollingStock {
    /// air_resistance :: double
    air_resistance: f64,

    /// connection_distance :: double
    connection_distance: f64,

    /// joint_distance :: double
    joint_distance: f64,

    /// max_speed :: double
    max_speed: f64,

    /// pictures :: RotatedSprite
    pictures: RotatedSprite,

    /// vertical_selection_shift :: double
    vertical_selection_shift: f64,

    /// allow_manual_color :: bool (optional)
    allow_manual_color: Option<bool>,

    /// allow_robot_dispatch_in_automatic_mode :: bool (optional)
    allow_robot_dispatch_in_automatic_mode: Option<bool>,

    /// back_light :: LightDefinition (optional)
    back_light: Option<LightDefinition>,

    /// color :: Color (optional)
    color: Option<Color>,

    /// drive_over_tie_trigger :: TriggerEffect (optional)
    drive_over_tie_trigger: Option<TriggerEffect>,

    /// horizontal_doors :: Animation (optional)
    horizontal_doors: Option<Animation>,

    /// stand_by_light :: LightDefinition (optional)
    stand_by_light: Option<LightDefinition>,

    /// tie_distance :: double (optional)
    tie_distance: Option<f64>,

    /// vertical_doors :: Animation (optional)
    vertical_doors: Option<Animation>,

    /// wheels :: RotatedSprite (optional)
    wheels: Option<RotatedSprite>,
}

impl Prototype for RollingStock {
    const TYPE: Option<&'static str> = Some("None");
}
