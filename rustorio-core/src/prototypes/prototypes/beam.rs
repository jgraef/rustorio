use serde::{Deserialize, Serialize};

use crate::prototypes::{Prototype, Visitor};
use crate::types::*; // TODO: Import only specific types

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Beam {
    /// body :: AnimationVariations
    body: AnimationVariations,

    /// damage_interval :: uint32
    damage_interval: u32,

    /// head :: Animation
    head: Animation,

    /// tail :: Animation
    tail: Animation,

    /// width :: double
    width: f64,

    /// action :: Trigger (optional)
    action: Option<Trigger>,

    /// action_triggered_automatically :: bool (optional)
    action_triggered_automatically: Option<bool>,

    /// body_light :: AnimationVariations (optional)
    body_light: Option<AnimationVariations>,

    /// ending :: Animation (optional)
    ending: Option<Animation>,

    /// ending_light :: Animation (optional)
    ending_light: Option<Animation>,

    /// ground_light_animations :: table (optional)
    ground_light_animations: Option<Vec<Todo>>,

    /// head_light :: Animation (optional)
    head_light: Option<Animation>,

    /// light_animations :: table (optional)
    light_animations: Option<Vec<Todo>>,

    /// random_end_animation_rotation :: bool (optional)
    random_end_animation_rotation: Option<bool>,

    /// random_target_offset :: bool (optional)
    random_target_offset: Option<bool>,

    /// start :: Animation (optional)
    start: Option<Animation>,

    /// start_light :: Animation (optional)
    start_light: Option<Animation>,

    /// tail_light :: Animation (optional)
    tail_light: Option<Animation>,

    /// target_offset :: vector (optional)
    target_offset: Option<Vector2<f32>>,

    /// transparent_start_end_animations :: bool (optional)
    transparent_start_end_animations: Option<bool>,
}

impl Prototype for Beam {
    const TYPE: Option<&'static str> = Some("beam");
}
