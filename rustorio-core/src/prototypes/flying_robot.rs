use serde::{Serialize, Deserialize};
        
use super::{Prototype, type_stubs::*};
        
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FlyingRobot {
    /// speed :: double
    speed: f64,    

    /// energy_per_move :: Energy (optional)
    energy_per_move: Option<Energy>,    

    /// energy_per_tick :: Energy (optional)
    energy_per_tick: Option<Energy>,    

    /// max_energy :: Energy (optional)
    max_energy: Option<Energy>,    

    /// max_speed :: double (optional)
    max_speed: Option<f64>,    

    /// max_to_charge :: float (optional)
    max_to_charge: Option<f32>,    

    /// min_to_charge :: float (optional)
    min_to_charge: Option<f32>,    

    /// speed_multiplier_when_out_of_energy :: float (optional)
    speed_multiplier_when_out_of_energy: Option<f32>,    

}

impl Prototype for FlyingRobot {
    const TYPE: Option<&'static str> = Some("None");
}


