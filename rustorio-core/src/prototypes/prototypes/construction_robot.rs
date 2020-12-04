use serde::{Deserialize, Serialize};

use crate::prototypes::{Prototype, Visitor};
use crate::types::*; // TODO: Import only specific types

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConstructionRobot {
    /// construction_vector :: vector
    construction_vector: Vector2<f32>,

    /// smoke :: Animation (optional)
    smoke: Option<Animation>,

    /// sparks :: AnimationVariations (optional)
    sparks: Option<AnimationVariations>,

    /// working :: RotatedAnimation (optional)
    working: Option<RotatedAnimation>,

    /// working_light :: LightDefinition (optional)
    working_light: Option<LightDefinition>,
}

impl Prototype for ConstructionRobot {
    const TYPE: Option<&'static str> = Some("construction-robot");
}
