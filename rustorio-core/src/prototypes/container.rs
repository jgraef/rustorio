use serde::{Serialize, Deserialize};
        
use super::{Prototype, type_stubs::*};
        
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Container {
    /// inventory_size :: uint16
    inventory_size: u16,    

    /// picture :: Sprite
    picture: Sprite,    

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

    /// enable_inventory_bar :: bool (optional)
    enable_inventory_bar: Option<bool>,    

    /// scale_info_icons :: bool (optional)
    scale_info_icons: Option<bool>,    

}

impl Prototype for Container {
    const TYPE: Option<&'static str> = Some("container");
}


