use serde::{Serialize, Deserialize};
        
use super::{Prototype, type_stubs::*};
        
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ElectricEnergyInterface {
    /// energy_source :: EnergySource
    energy_source: EnergySource,    

    /// animation :: Animation (optional)
    animation: Option<Animation>,    

    /// animations :: Animation4Way (optional)
    animations: Option<Animation4Way>,    

    /// continuous_animation :: bool (optional)
    continuous_animation: Option<bool>,    

    /// energy_production :: Energy (optional)
    energy_production: Option<Energy>,    

    /// energy_usage :: Energy (optional)
    energy_usage: Option<Energy>,    

    /// gui_mode :: string (optional)
    gui_mode: Option<String>,    

    /// light :: LightDefinition (optional)
    light: Option<LightDefinition>,    

    /// picture :: Sprite (optional)
    picture: Option<Sprite>,    

    /// pictures :: Sprite4Way (optional)
    pictures: Option<Sprite4Way>,    

    /// render_layer :: RenderLayer (optional)
    render_layer: Option<RenderLayer>,    

}

impl Prototype for ElectricEnergyInterface {
    const TYPE: Option<&'static str> = Some("electric-energy-interface");
}


