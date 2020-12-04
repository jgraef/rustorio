use serde::{Deserialize, Serialize};

use crate::prototypes::{Prototype, Visitor};
use crate::types::*; // TODO: Import only specific types

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TransportBelt {
    /// connector_frame_sprites :: TransportBeltConnectorFrame
    connector_frame_sprites: TransportBeltConnectorFrame,

    /// animations :: RotatedAnimation (optional)
    animations: Option<RotatedAnimation>,

    /// belt_animation_set :: table (optional)
    belt_animation_set: Option<Vec<Todo>>,

    /// circuit_connector_sprites :: Array of CircuitConnectorSprites (optional)
    circuit_connector_sprites: Option<Vec<CircuitConnectorSprites>>,

    /// circuit_wire_connection_point :: Array of WireConnectionPoint (optional)
    circuit_wire_connection_point: Option<Vec<WireConnectionPoint>>,

    /// circuit_wire_max_distance :: double (optional)
    circuit_wire_max_distance: Option<f64>,

    /// draw_circuit_wires :: bool (optional)
    draw_circuit_wires: Option<bool>,

    /// draw_copper_wires :: bool (optional)
    draw_copper_wires: Option<bool>,
}

impl Prototype for TransportBelt {
    const TYPE: Option<&'static str> = Some("transport-belt");
}
