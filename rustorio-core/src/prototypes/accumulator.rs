use serde::{Deserialize, Serialize};

use super::Prototype;
use crate::types::*; // TODO: Import only specific types

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Accumulator {
    /// charge_cooldown :: uint16
    charge_cooldown: u16,

    /// discharge_cooldown :: uint16
    discharge_cooldown: u16,

    /// energy_source :: EnergySource
    energy_source: EnergySource,

    /// picture :: Sprite
    picture: Sprite,

    /// charge_animation :: Animation (optional)
    charge_animation: Option<Animation>,

    /// charge_light :: LightDefinition (optional)
    charge_light: Option<LightDefinition>,

    /// circuit_connector_sprites :: CircuitConnectorSprites (optional)
    circuit_connector_sprites: Option<CircuitConnectorSprites>,

    /// circuit_wire_connection_point :: WireConnectionPoint (optional)
    circuit_wire_connection_point: Option<WireConnectionPoint>,

    /// circuit_wire_max_distance :: double (optional)
    circuit_wire_max_distance: Option<f64>,

    /// default_output_signal :: SignalIDConnector (optional)
    default_output_signal: Option<SignalIDConnector>,

    /// discharge_animation :: Animation (optional)
    discharge_animation: Option<Animation>,

    /// discharge_light :: LightDefinition (optional)
    discharge_light: Option<LightDefinition>,

    /// draw_circuit_wires :: bool (optional)
    draw_circuit_wires: Option<bool>,

    /// draw_copper_wires :: bool (optional)
    draw_copper_wires: Option<bool>,
}

impl Prototype for Accumulator {
    const TYPE: Option<&'static str> = Some("accumulator");
}
