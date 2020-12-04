use serde::{Deserialize, Serialize};

use crate::prototypes::{Prototype, Visitor};
use crate::types::*; // TODO: Import only specific types

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TileEffect {
    /// animation_scale :: array of one or two float
    animation_scale: Todo,

    /// animation_speed :: float
    animation_speed: f32,

    /// dark_threshold :: array of one or two float
    dark_threshold: Todo,

    /// foam_color :: Color
    foam_color: Color,

    /// foam_color_multiplier :: float
    foam_color_multiplier: f32,

    /// name :: string
    name: String,

    /// reflection_threshold :: array of one or two float
    reflection_threshold: Todo,

    /// specular_lightness :: Color
    specular_lightness: Color,

    /// specular_threshold :: array of one or two float
    specular_threshold: Todo,

    /// texture :: Sprite
    texture: Sprite,

    /// tick_scale :: float
    tick_scale: f32,

    /// type :: string
    r#type: String,

    /// far_zoom :: float (optional)
    far_zoom: Option<f32>,

    /// near_zoom :: float (optional)
    near_zoom: Option<f32>,
}

impl Prototype for TileEffect {
    const TYPE: Option<&'static str> = Some("tile-effect");
}
