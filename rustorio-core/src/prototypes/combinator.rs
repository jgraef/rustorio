use serde::{Deserialize, Serialize};

use super::Prototype;
use crate::types::*; // TODO: Import only specific types

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Combinator {
    /// active_energy_usage :: Energy
    active_energy_usage: Energy,

    /// activity_led_light_offsets :: table of vector
    activity_led_light_offsets: Vec<Vector2<f32>>,

    /// activity_led_sprites :: Sprite4Way
    activity_led_sprites: Sprite4Way,

    /// energy_source :: EnergySource
    energy_source: EnergySource,

    /// input_connection_bounding_box :: BoundingBox
    input_connection_bounding_box: BoundingBox,

    /// input_connection_points :: table of WireConnectionPoint
    input_connection_points: Vec<WireConnectionPoint>,

    /// output_connection_bounding_box :: BoundingBox
    output_connection_bounding_box: BoundingBox,

    /// output_connection_points :: table of WireConnectionPoint
    output_connection_points: Vec<WireConnectionPoint>,

    /// screen_light_offsets :: table of vector
    screen_light_offsets: Vec<Vector2<f32>>,

    /// sprites :: Sprite4Way
    sprites: Sprite4Way,

    /// activity_led_hold_time :: uint8 (optional)
    activity_led_hold_time: Option<u8>,

    /// activity_led_light :: LightDefinition (optional)
    activity_led_light: Option<LightDefinition>,

    /// circuit_wire_max_distance :: double (optional)
    circuit_wire_max_distance: Option<f64>,

    /// draw_circuit_wires :: bool (optional)
    draw_circuit_wires: Option<bool>,

    /// draw_copper_wires :: bool (optional)
    draw_copper_wires: Option<bool>,

    /// screen_light :: LightDefinition (optional)
    screen_light: Option<LightDefinition>,
}

impl Prototype for Combinator {
    const TYPE: Option<&'static str> = Some("None");
}
