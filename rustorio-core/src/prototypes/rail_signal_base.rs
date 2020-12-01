use serde::{Deserialize, Serialize};

use super::Prototype;
use crate::types::*; // TODO: Import only specific types

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RailSignalBase {
    /// animation :: RotatedAnimation
    animation: RotatedAnimation,

    /// circuit_connector_sprites :: table of CircuitConnectorSprites (optional)
    circuit_connector_sprites: Option<Vec<CircuitConnectorSprites>>,

    /// circuit_wire_connection_points :: table of WireConnectionPoint (optional)
    circuit_wire_connection_points: Option<Vec<WireConnectionPoint>>,

    /// circuit_wire_max_distance :: double (optional)
    circuit_wire_max_distance: Option<f64>,

    /// default_green_output_signal :: SignalIDConnector (optional)
    default_green_output_signal: Option<SignalIDConnector>,

    /// default_orange_output_signal :: SignalIDConnector (optional)
    default_orange_output_signal: Option<SignalIDConnector>,

    /// default_red_output_signal :: SignalIDConnector (optional)
    default_red_output_signal: Option<SignalIDConnector>,

    /// draw_circuit_wires :: bool (optional)
    draw_circuit_wires: Option<bool>,

    /// draw_copper_wires :: bool (optional)
    draw_copper_wires: Option<bool>,

    /// green_light :: LightDefinition (optional)
    green_light: Option<LightDefinition>,

    /// orange_light :: LightDefinition (optional)
    orange_light: Option<LightDefinition>,

    /// rail_piece :: Animation (optional)
    rail_piece: Option<Animation>,

    /// red_light :: LightDefinition (optional)
    red_light: Option<LightDefinition>,
}

impl Prototype for RailSignalBase {
    const TYPE: Option<&'static str> = Some("None");
}
