use serde::{Serialize, Deserialize};
        
use super::{Prototype, type_stubs::*};
        
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Radar {
    /// energy_per_nearby_scan :: Energy
    energy_per_nearby_scan: Energy,    

    /// energy_per_sector :: Energy
    energy_per_sector: Energy,    

    /// energy_source :: EnergySource
    energy_source: EnergySource,    

    /// energy_usage :: Energy
    energy_usage: Energy,    

    /// max_distance_of_nearby_sector_revealed :: uint32
    max_distance_of_nearby_sector_revealed: u32,    

    /// max_distance_of_sector_revealed :: uint32
    max_distance_of_sector_revealed: u32,    

    /// pictures :: RotatedSprite
    pictures: RotatedSprite,    

    /// radius_minimap_visualisation_color :: Color (optional)
    radius_minimap_visualisation_color: Option<Color>,    

    /// rotation_speed :: double (optional)
    rotation_speed: Option<f64>,    

}

impl Prototype for Radar {
    const TYPE: Option<&'static str> = Some("radar");
}


