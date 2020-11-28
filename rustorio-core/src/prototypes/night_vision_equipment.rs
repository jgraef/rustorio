use serde::{Serialize, Deserialize};
        
use super::{Prototype, type_stubs::*};
        
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NightVisionEquipment {
    /// color_lookup :: DaytimeColorLookupTable
    color_lookup: DaytimeColorLookupTable,    

    /// energy_input :: Energy
    energy_input: Energy,    

    /// activate_sound :: Sound (optional)
    activate_sound: Option<Sound>,    

    /// darkness_to_turn_on :: float (optional)
    darkness_to_turn_on: Option<f32>,    

    /// deactivate_sound :: Sound (optional)
    deactivate_sound: Option<Sound>,    

}

impl Prototype for NightVisionEquipment {
    const TYPE: Option<&'static str> = Some("night-vision-equipment");
}


