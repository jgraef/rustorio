use serde::{Serialize, Deserialize};
        
use super::{Prototype, type_stubs::*};
        
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ElectricPole {
    /// connection_points :: table of WireConnectionPoint
    connection_points: Vec<WireConnectionPoint>,    

    /// pictures :: RotatedSprite
    pictures: RotatedSprite,    

    /// supply_area_distance :: double
    supply_area_distance: f64,    

    /// active_picture :: Sprite (optional)
    active_picture: Option<Sprite>,    

    /// draw_circuit_wires :: bool (optional)
    draw_circuit_wires: Option<bool>,    

    /// draw_copper_wires :: bool (optional)
    draw_copper_wires: Option<bool>,    

    /// light :: LightDefinition (optional)
    light: Option<LightDefinition>,    

    /// maximum_wire_distance :: double (optional)
    maximum_wire_distance: Option<f64>,    

    /// radius_visualisation_picture :: Sprite (optional)
    radius_visualisation_picture: Option<Sprite>,    

    /// track_coverage_during_build_by_moving :: bool (optional)
    track_coverage_during_build_by_moving: Option<bool>,    

}

impl Prototype for ElectricPole {
    const TYPE: Option<&'static str> = Some("electric-pole");
}


