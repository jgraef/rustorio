use serde::{Serialize, Deserialize};
        
use super::{Prototype, type_stubs::*};
        
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Wall {
    /// pictures :: table
    pictures: Vec<Todo>,

    /// circuit_connector_sprites :: CircuitConnectorSprites (optional)
    circuit_connector_sprites: Option<CircuitConnectorSprites>,    

    /// circuit_wire_connection_point :: WireConnectionPoint (optional)
    circuit_wire_connection_point: Option<WireConnectionPoint>,    

    /// circuit_wire_max_distance :: double (optional)
    circuit_wire_max_distance: Option<f64>,    

    /// connected_gate_visualization :: Sprite (optional)
    connected_gate_visualization: Option<Sprite>,    

    /// default_output_signal :: SignalIDConnector (optional)
    default_output_signal: Option<SignalIDConnector>,    

    /// draw_circuit_wires :: bool (optional)
    draw_circuit_wires: Option<bool>,    

    /// draw_copper_wires :: bool (optional)
    draw_copper_wires: Option<bool>,    

    /// visual_merge_group :: uint32 (optional)
    visual_merge_group: Option<u32>,    

    /// wall_diode_green :: Sprite4Way (optional)
    wall_diode_green: Option<Sprite4Way>,    

    /// wall_diode_green_light_bottom :: LightDefinition (optional)
    wall_diode_green_light_bottom: Option<LightDefinition>,    

    /// wall_diode_green_light_left :: LightDefinition (optional)
    wall_diode_green_light_left: Option<LightDefinition>,    

    /// wall_diode_green_light_right :: LightDefinition (optional)
    wall_diode_green_light_right: Option<LightDefinition>,    

    /// wall_diode_green_light_top :: LightDefinition (optional)
    wall_diode_green_light_top: Option<LightDefinition>,    

    /// wall_diode_red :: Sprite4Way (optional)
    wall_diode_red: Option<Sprite4Way>,    

    /// wall_diode_red_light_bottom :: LightDefinition (optional)
    wall_diode_red_light_bottom: Option<LightDefinition>,    

    /// wall_diode_red_light_left :: LightDefinition (optional)
    wall_diode_red_light_left: Option<LightDefinition>,    

    /// wall_diode_red_light_right :: LightDefinition (optional)
    wall_diode_red_light_right: Option<LightDefinition>,    

    /// wall_diode_red_light_top :: LightDefinition (optional)
    wall_diode_red_light_top: Option<LightDefinition>,    

}

impl Prototype for Wall {
    const TYPE: Option<&'static str> = Some("wall");
}


