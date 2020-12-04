use serde::{Deserialize, Serialize};

use crate::prototypes::{Prototype, Visitor};
use crate::types::*; // TODO: Import only specific types

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Explosion {
    /// animations :: AnimationVariations
    animations: AnimationVariations,

    /// beam :: bool (optional)
    beam: Option<bool>,

    /// correct_rotation :: bool (optional)
    correct_rotation: Option<bool>,

    /// height :: float (optional)
    height: Option<f32>,

    /// light :: LightDefinition (optional)
    light: Option<LightDefinition>,

    /// render_layer :: RenderLayer (optional)
    render_layer: Option<RenderLayer>,

    /// rotate :: bool (optional)
    rotate: Option<bool>,

    /// smoke :: string (optional)
    smoke: Option<String>,

    /// smoke_count :: uint16 (optional)
    smoke_count: Option<u16>,

    /// smoke_slow_down_factor :: float (optional)
    smoke_slow_down_factor: Option<f32>,

    /// sound :: Sound (optional)
    sound: Option<Sound>,
}

impl Prototype for Explosion {
    const TYPE: Option<&'static str> = Some("explosion");
}
