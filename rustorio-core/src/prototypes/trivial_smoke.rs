use serde::{Serialize, Deserialize};
        
use super::{Prototype, type_stubs::*};
        
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TrivialSmoke {
    /// animation :: Animation
    animation: Animation,    

    /// duration :: uint32
    duration: u32,    

    /// affected_by_wind :: bool (optional)
    affected_by_wind: Option<bool>,    

    /// color :: Color (optional)
    color: Option<Color>,    

    /// cyclic :: bool (optional)
    cyclic: Option<bool>,    

    /// end_scale :: double (optional)
    end_scale: Option<f64>,    

    /// fade_away_duration :: uint32 (optional)
    fade_away_duration: Option<u32>,    

    /// fade_in_duration :: uint32 (optional)
    fade_in_duration: Option<u32>,    

    /// glow_animation :: Animation (optional)
    glow_animation: Option<Animation>,    

    /// glow_fade_away_duration :: uint32 (optional)
    glow_fade_away_duration: Option<u32>,    

    /// movement_slow_down_factor :: double (optional)
    movement_slow_down_factor: Option<f64>,    

    /// render_layer :: RenderLayer (optional)
    render_layer: Option<RenderLayer>,    

    /// show_when_smoke_off :: bool (optional)
    show_when_smoke_off: Option<bool>,    

    /// spread_duration :: uint32 (optional)
    spread_duration: Option<u32>,    

    /// start_scale :: double (optional)
    start_scale: Option<f64>,    

}

impl Prototype for TrivialSmoke {
    const TYPE: Option<&'static str> = Some("trivial-smoke");
}


