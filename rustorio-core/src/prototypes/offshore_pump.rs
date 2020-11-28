use serde::{Serialize, Deserialize};
        
use super::{Prototype, type_stubs::*};
        
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OffshorePump {
    /// fluid :: string
    fluid: String,    

    /// fluid_box :: FluidBox
    fluid_box: FluidBox,    

    /// pumping_speed :: float
    pumping_speed: f32,    

    /// adjacent_tile_collision_box :: BoundingBox (optional)
    adjacent_tile_collision_box: Option<BoundingBox>,    

    /// adjacent_tile_collision_mask :: CollisionMask (optional)
    adjacent_tile_collision_mask: Option<CollisionMask>,    

    /// adjacent_tile_collision_test :: CollisionMask (optional)
    adjacent_tile_collision_test: Option<CollisionMask>,    

    /// always_draw_fluid :: bool (optional)
    always_draw_fluid: Option<bool>,    

    /// center_collision_mask :: CollisionMask (optional)
    center_collision_mask: Option<CollisionMask>,    

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

    /// fluid_box_tile_collision_test :: CollisionMask (optional)
    fluid_box_tile_collision_test: Option<CollisionMask>,    

    /// graphics_set :: table (optional)
    graphics_set: Option<Vec<Todo>>,

    /// min_perceived_performance :: float (optional)
    min_perceived_performance: Option<f32>,    

    /// picture :: Sprite4Way (optional)
    picture: Option<Sprite4Way>,    

    /// placeable_position_visualization :: Sprite (optional)
    placeable_position_visualization: Option<Sprite>,    

    /// remove_on_tile_collision :: bool (optional)
    remove_on_tile_collision: Option<bool>,    

}

impl Prototype for OffshorePump {
    const TYPE: Option<&'static str> = Some("offshore-pump");
}


