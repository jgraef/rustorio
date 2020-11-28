use serde::{Serialize, Deserialize};
        
use super::{Prototype, type_stubs::*};
        
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Inserter {
    /// energy_source :: EnergySource
    energy_source: EnergySource,    

    /// extension_speed :: double
    extension_speed: f64,    

    /// hand_base_picture :: Sprite
    hand_base_picture: Sprite,    

    /// hand_base_shadow :: Sprite
    hand_base_shadow: Sprite,    

    /// hand_closed_picture :: Sprite
    hand_closed_picture: Sprite,    

    /// hand_closed_shadow :: Sprite
    hand_closed_shadow: Sprite,    

    /// hand_open_picture :: Sprite
    hand_open_picture: Sprite,    

    /// hand_open_shadow :: Sprite
    hand_open_shadow: Sprite,    

    /// insert_position :: vector
    insert_position: Vector2<f32>,    

    /// pickup_position :: vector
    pickup_position: Vector2<f32>,    

    /// platform_picture :: Sprite4Way
    platform_picture: Sprite4Way,    

    /// rotation_speed :: double
    rotation_speed: f64,    

    /// allow_burner_leech :: bool (optional)
    allow_burner_leech: Option<bool>,    

    /// allow_custom_vectors :: bool (optional)
    allow_custom_vectors: Option<bool>,    

    /// chases_belt_items :: bool (optional)
    chases_belt_items: Option<bool>,    

    /// circuit_connector_sprites :: table of CircuitConnectorSprites (optional)
    circuit_connector_sprites: Option<Vec<CircuitConnectorSprites>>,    

    /// circuit_wire_connection_points :: table of WireConnectionPoint (optional)
    circuit_wire_connection_points: Option<Vec<WireConnectionPoint>>,    

    /// circuit_wire_max_distance :: double (optional)
    circuit_wire_max_distance: Option<f64>,    

    /// default_stack_control_input_signal :: SignalIDConnector (optional)
    default_stack_control_input_signal: Option<SignalIDConnector>,    

    /// draw_circuit_wires :: bool (optional)
    draw_circuit_wires: Option<bool>,    

    /// draw_copper_wires :: bool (optional)
    draw_copper_wires: Option<bool>,    

    /// draw_held_item :: bool (optional)
    draw_held_item: Option<bool>,    

    /// draw_inserter_arrow :: bool (optional)
    draw_inserter_arrow: Option<bool>,    

    /// energy_per_movement :: Energy (optional)
    energy_per_movement: Option<Energy>,    

    /// energy_per_rotation :: Energy (optional)
    energy_per_rotation: Option<Energy>,    

    /// filter_count :: uint8 (optional)
    filter_count: Option<u8>,    

    /// hand_size :: double (optional)
    hand_size: Option<f64>,    

    /// stack :: bool (optional)
    stack: Option<bool>,    

    /// use_easter_egg :: bool (optional)
    use_easter_egg: Option<bool>,    

}

impl Prototype for Inserter {
    const TYPE: Option<&'static str> = Some("inserter");
}


