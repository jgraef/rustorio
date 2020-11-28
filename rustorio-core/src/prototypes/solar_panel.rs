use serde::{Serialize, Deserialize};
        
use super::{Prototype, type_stubs::*};
        
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SolarPanel {
    /// energy_source :: EnergySource
    energy_source: EnergySource,    

    /// picture :: SpriteVariations
    picture: SpriteVariations,    

    /// production :: Energy
    production: Energy,    

    /// overlay :: SpriteVariations (optional)
    overlay: Option<SpriteVariations>,    

}

impl Prototype for SolarPanel {
    const TYPE: Option<&'static str> = Some("solar-panel");
}


