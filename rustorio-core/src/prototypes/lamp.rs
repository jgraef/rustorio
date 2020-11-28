use serde::{Serialize, Deserialize};
        
use super::{Prototype, type_stubs::*};
        
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Lamp {
    /// energy_source :: EnergySource
    energy_source: EnergySource,    

    /// energy_usage_per_tick :: Energy
    energy_usage_per_tick: Energy,    

    /// picture_off :: Sprite
    picture_off: Sprite,    

    /// picture_on :: Sprite
    picture_on: Sprite,    

    /// always_on :: bool (optional)
    always_on: Option<bool>,    

    /// circuit_connector_sprites :: CircuitConnectorSprites (optional)
    circuit_connector_sprites: Option<CircuitConnectorSprites>,    

    /// circuit_wire_connection_point :: WireConnectionPoint (optional)
    circuit_wire_connection_point: Option<WireConnectionPoint>,    

    /// circuit_wire_max_distance :: double (optional)
    circuit_wire_max_distance: Option<f64>,    

    /// darkness_for_all_lamps_off :: float (optional)
    darkness_for_all_lamps_off: Option<f32>,    

    /// darkness_for_all_lamps_on :: float (optional)
    darkness_for_all_lamps_on: Option<f32>,    

    /// draw_circuit_wires :: bool (optional)
    draw_circuit_wires: Option<bool>,    

    /// draw_copper_wires :: bool (optional)
    draw_copper_wires: Option<bool>,    

    /// glow_color_intensity :: float (optional)
    glow_color_intensity: Option<f32>,    

    /// glow_render_mode :: string (optional)
    glow_render_mode: Option<String>,    

    /// glow_size :: float (optional)
    glow_size: Option<f32>,    

    /// light :: LightDefinition (optional)
    light: Option<LightDefinition>,    

    /// light_when_colored :: LightDefinition (optional)
    light_when_colored: Option<LightDefinition>,    

    /// signal_to_color_mapping :: table of SignalColorMapping (optional)
    signal_to_color_mapping: Option<Vec<SignalColorMapping>>,    

}

impl Prototype for Lamp {
    const TYPE: Option<&'static str> = Some("lamp");
}


