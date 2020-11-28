use serde::{Serialize, Deserialize};
        
use super::{Prototype, type_stubs::*};
        
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Pump {
    /// animations :: Animation4Way
    animations: Animation4Way,    

    /// energy_source :: EnergySource
    energy_source: EnergySource,    

    /// energy_usage :: Energy
    energy_usage: Energy,    

    /// fluid_box :: FluidBox
    fluid_box: FluidBox,    

    /// pumping_speed :: double
    pumping_speed: f64,    

    /// circuit_connector_sprites :: table of CircuitConnectorSprites (optional)
    circuit_connector_sprites: Option<Vec<CircuitConnectorSprites>>,    

    /// circuit_wire_connection_points :: table of WireConnectionPoint (optional)
    circuit_wire_connection_points: Option<Vec<WireConnectionPoint>>,    

    /// circuit_wire_max_distance :: double (optional)
    circuit_wire_max_distance: Option<f64>,    

    /// draw_circuit_wires :: bool (optional)
    draw_circuit_wires: Option<bool>,    

    /// draw_copper_wires :: bool (optional)
    draw_copper_wires: Option<bool>,    

    /// fluid_animation :: Animation4Way (optional)
    fluid_animation: Option<Animation4Way>,    

    /// fluid_wagon_connector_alignment_tolerance :: double (optional)
    fluid_wagon_connector_alignment_tolerance: Option<f64>,    

    /// fluid_wagon_connector_frame_count :: uint8 (optional)
    fluid_wagon_connector_frame_count: Option<u8>,    

    /// fluid_wagon_connector_graphics :: table of PumpConnectorGraphics (optional)
    fluid_wagon_connector_graphics: Option<Vec<PumpConnectorGraphics>>,    

    /// fluid_wagon_connector_speed :: double (optional)
    fluid_wagon_connector_speed: Option<f64>,    

    /// glass_pictures :: Sprite4Way (optional)
    glass_pictures: Option<Sprite4Way>,    

}

impl Prototype for Pump {
    const TYPE: Option<&'static str> = Some("pump");
}


