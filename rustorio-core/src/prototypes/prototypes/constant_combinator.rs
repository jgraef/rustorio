use serde::{Deserialize, Serialize};

use crate::prototypes::{Prototype, Visitor};
use crate::types::*; // TODO: Import only specific types

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConstantCombinator {
    /// activity_led_light_offsets :: table of vector
    activity_led_light_offsets: Vec<Vector2<f32>>,

    /// activity_led_sprites :: Sprite4Way
    activity_led_sprites: Sprite4Way,

    /// circuit_wire_connection_points :: table of WireConnectionPoint
    circuit_wire_connection_points: Vec<WireConnectionPoint>,

    /// item_slot_count :: uint32
    item_slot_count: u32,

    /// sprites :: Sprite4Way
    sprites: Sprite4Way,

    /// activity_led_light :: LightDefinition (optional)
    activity_led_light: Option<LightDefinition>,

    /// circuit_wire_max_distance :: double (optional)
    circuit_wire_max_distance: Option<f64>,

    /// draw_circuit_wires :: bool (optional)
    draw_circuit_wires: Option<bool>,

    /// draw_copper_wires :: bool (optional)
    draw_copper_wires: Option<bool>,
}

impl Prototype for ConstantCombinator {
    const TYPE: Option<&'static str> = Some("constant-combinator");
}
