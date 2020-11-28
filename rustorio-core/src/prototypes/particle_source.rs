use serde::{Serialize, Deserialize};
        
use super::{Prototype, type_stubs::*};
        
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ParticleSource {
    /// height :: float
    height: f32,    

    /// horizontal_speed :: float
    horizontal_speed: f32,    

    /// particle :: string
    particle: String,    

    /// time_before_start :: float
    time_before_start: f32,    

    /// time_to_live :: float
    time_to_live: f32,    

    /// vertical_speed :: float
    vertical_speed: f32,    

    /// height_deviation :: float (optional)
    height_deviation: Option<f32>,    

    /// horizontal_speed_deviation :: float (optional)
    horizontal_speed_deviation: Option<f32>,    

    /// time_before_start_deviation :: float (optional)
    time_before_start_deviation: Option<f32>,    

    /// time_to_live_deviation :: float (optional)
    time_to_live_deviation: Option<f32>,    

    /// vertical_speed_deviation :: float (optional)
    vertical_speed_deviation: Option<f32>,    

}

impl Prototype for ParticleSource {
    const TYPE: Option<&'static str> = Some("particle-source");
}


