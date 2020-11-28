use serde::{Serialize, Deserialize};
        
use super::{Prototype, type_stubs::*};
        
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Lab {
    /// energy_source :: EnergySource
    energy_source: EnergySource,    

    /// energy_usage :: Energy
    energy_usage: Energy,    

    /// inputs :: table of string
    inputs: Vec<String>,

    /// off_animation :: Animation
    off_animation: Animation,    

    /// on_animation :: Animation
    on_animation: Animation,    

    /// allowed_effects :: EffectTypeLimitation (optional)
    allowed_effects: Option<EffectTypeLimitation>,    

    /// base_productivity :: float (optional)
    base_productivity: Option<f32>,    

    /// entity_info_icon_shift :: vector (optional)
    entity_info_icon_shift: Option<Vector2<f32>>,    

    /// light :: LightDefinition (optional)
    light: Option<LightDefinition>,    

    /// module_specification :: ModuleSpecification (optional)
    module_specification: Option<ModuleSpecification>,    

    /// researching_speed :: double (optional)
    researching_speed: Option<f64>,    

}

impl Prototype for Lab {
    const TYPE: Option<&'static str> = Some("lab");
}


