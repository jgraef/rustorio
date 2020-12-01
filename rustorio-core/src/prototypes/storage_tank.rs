use serde::{Deserialize, Serialize};

use super::Prototype;
use crate::types::*; // TODO: Import only specific types

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StorageTank {
    /// flow_length_in_ticks :: uint32
    flow_length_in_ticks: u32,

    /// fluid_box :: FluidBox
    fluid_box: FluidBox,

    /// pictures :: table
    pictures: Vec<Todo>,

    /// window_bounding_box :: BoundingBox
    window_bounding_box: BoundingBox,

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

    /// scale_info_icons :: bool (optional)
    scale_info_icons: Option<bool>,

    /// two_direction_only :: bool (optional)
    two_direction_only: Option<bool>,
}

impl Prototype for StorageTank {
    const TYPE: Option<&'static str> = Some("storage-tank");
}
