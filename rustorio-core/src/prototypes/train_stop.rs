use serde::{Serialize, Deserialize};
        
use super::{Prototype, type_stubs::*};
        
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TrainStop {
    /// animation_ticks_per_frame :: uint32
    animation_ticks_per_frame: u32,    

    /// animations :: Animation4Way (optional)
    animations: Option<Animation4Way>,    

    /// chart_name :: bool (optional)
    chart_name: Option<bool>,    

    /// circuit_connector_sprites :: table of CircuitConnectorSprites (optional)
    circuit_connector_sprites: Option<Vec<CircuitConnectorSprites>>,    

    /// circuit_wire_connection_points :: table of WireConnectionPoint (optional)
    circuit_wire_connection_points: Option<Vec<WireConnectionPoint>>,    

    /// circuit_wire_max_distance :: double (optional)
    circuit_wire_max_distance: Option<f64>,    

    /// color :: Color (optional)
    color: Option<Color>,    

    /// default_train_stopped_signal :: SignalIDConnector (optional)
    default_train_stopped_signal: Option<SignalIDConnector>,    

    /// draw_circuit_wires :: bool (optional)
    draw_circuit_wires: Option<bool>,    

    /// draw_copper_wires :: bool (optional)
    draw_copper_wires: Option<bool>,    

    /// drawing_boxes :: table (optional)
    drawing_boxes: Option<Vec<Todo>>,

    /// light1 :: table (optional)
    light1: Option<Vec<Todo>>,

    /// light2 :: table (optional)
    light2: Option<Vec<Todo>>,

    /// rail_overlay_animations :: Animation4Way (optional)
    rail_overlay_animations: Option<Animation4Way>,    

    /// top_animations :: Animation4Way (optional)
    top_animations: Option<Animation4Way>,    

}

impl Prototype for TrainStop {
    const TYPE: Option<&'static str> = Some("train-stop");
}


