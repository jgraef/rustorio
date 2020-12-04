use serde::{Deserialize, Serialize};

use crate::prototypes::{Prototype, Visitor};
use crate::types::*; // TODO: Import only specific types

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProgrammableSpeaker {
    /// energy_source :: EnergySource
    energy_source: EnergySource,

    /// energy_usage_per_tick :: Energy
    energy_usage_per_tick: Energy,

    /// instruments :: table
    instruments: Vec<Todo>,

    /// maximum_polyphony :: uint32
    maximum_polyphony: u32,

    /// sprite :: Sprite
    sprite: Sprite,

    /// audible_distance_modifier :: float (optional)
    audible_distance_modifier: Option<f32>,

    /// circuit_connector_sprites :: CircuitConnectorSprites (optional)
    circuit_connector_sprites: Option<CircuitConnectorSprites>,

    /// circuit_wire_connection_point :: WireConnectionPoint (optional)
    circuit_wire_connection_point: Option<WireConnectionPoint>,

    /// circuit_wire_max_distance :: double (optional)
    circuit_wire_max_distance: Option<f64>,

    /// draw_circuit_wires :: bool (optional)
    draw_circuit_wires: Option<bool>,

    /// draw_copper_wires :: bool (optional)
    draw_copper_wires: Option<bool>,
}

impl Prototype for ProgrammableSpeaker {
    const TYPE: Option<&'static str> = Some("programmable-speaker");
}
