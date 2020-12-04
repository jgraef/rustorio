use serde::{Deserialize, Serialize};

use crate::prototypes::{Prototype, Visitor};
use crate::types::*; // TODO: Import only specific types

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PowerSwitch {
    /// circuit_wire_connection_point :: WireConnectionPoint
    circuit_wire_connection_point: WireConnectionPoint,

    /// led_off :: Sprite
    led_off: Sprite,

    /// led_on :: Sprite
    led_on: Sprite,

    /// left_wire_connection_point :: WireConnectionPoint
    left_wire_connection_point: WireConnectionPoint,

    /// overlay_loop :: Animation
    overlay_loop: Animation,

    /// overlay_start :: Animation
    overlay_start: Animation,

    /// overlay_start_delay :: uint8
    overlay_start_delay: u8,

    /// power_on_animation :: Animation
    power_on_animation: Animation,

    /// right_wire_connection_point :: WireConnectionPoint
    right_wire_connection_point: WireConnectionPoint,

    /// draw_circuit_wires :: bool (optional)
    draw_circuit_wires: Option<bool>,

    /// draw_copper_wires :: bool (optional)
    draw_copper_wires: Option<bool>,

    /// wire_max_distance :: double (optional)
    wire_max_distance: Option<f64>,
}

impl Prototype for PowerSwitch {
    const TYPE: Option<&'static str> = Some("power-switch");
}
