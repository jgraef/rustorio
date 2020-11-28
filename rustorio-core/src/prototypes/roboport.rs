use serde::{Serialize, Deserialize};
        
use super::{Prototype, type_stubs::*};
        
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Roboport {
    /// base :: Sprite
    base: Sprite,    

    /// base_animation :: Animation
    base_animation: Animation,    

    /// base_patch :: Sprite
    base_patch: Sprite,    

    /// charge_approach_distance :: float
    charge_approach_distance: f32,    

    /// charging_energy :: Energy
    charging_energy: Energy,    

    /// construction_radius :: float
    construction_radius: f32,    

    /// door_animation_down :: Animation
    door_animation_down: Animation,    

    /// door_animation_up :: Animation
    door_animation_up: Animation,    

    /// energy_source :: EnergySource
    energy_source: EnergySource,    

    /// energy_usage :: Energy
    energy_usage: Energy,    

    /// logistics_radius :: float
    logistics_radius: f32,    

    /// material_slots_count :: ItemStackIndex
    material_slots_count: ItemStackIndex,    

    /// recharge_minimum :: Energy
    recharge_minimum: Energy,    

    /// recharging_animation :: Animation
    recharging_animation: Animation,    

    /// request_to_open_door_timeout :: uint32
    request_to_open_door_timeout: u32,    

    /// robot_slots_count :: ItemStackIndex
    robot_slots_count: ItemStackIndex,    

    /// spawn_and_station_height :: float
    spawn_and_station_height: f32,    

    /// charging_distance :: float (optional)
    charging_distance: Option<f32>,    

    /// charging_offsets :: table of vector (optional)
    charging_offsets: Option<Vec<Vector2<f32>>>,

    /// charging_station_count :: uint32 (optional)
    charging_station_count: Option<u32>,    

    /// charging_station_shift :: vector (optional)
    charging_station_shift: Option<Vector2<f32>>,    

    /// charging_threshold_distance :: float (optional)
    charging_threshold_distance: Option<f32>,    

    /// circuit_connector_sprites :: CircuitConnectorSprites (optional)
    circuit_connector_sprites: Option<CircuitConnectorSprites>,    

    /// circuit_wire_connection_point :: WireConnectionPoint (optional)
    circuit_wire_connection_point: Option<WireConnectionPoint>,    

    /// circuit_wire_max_distance :: double (optional)
    circuit_wire_max_distance: Option<f64>,    

    /// close_door_trigger_effect :: TriggerEffect (optional)
    close_door_trigger_effect: Option<TriggerEffect>,    

    /// default_available_construction_output_signal :: SignalIDConnector (optional)
    default_available_construction_output_signal: Option<SignalIDConnector>,    

    /// default_available_logistic_output_signal :: SignalIDConnector (optional)
    default_available_logistic_output_signal: Option<SignalIDConnector>,    

    /// default_total_construction_output_signal :: SignalIDConnector (optional)
    default_total_construction_output_signal: Option<SignalIDConnector>,    

    /// default_total_logistic_output_signal :: SignalIDConnector (optional)
    default_total_logistic_output_signal: Option<SignalIDConnector>,    

    /// draw_circuit_wires :: bool (optional)
    draw_circuit_wires: Option<bool>,    

    /// draw_construction_radius_visualization :: bool (optional)
    draw_construction_radius_visualization: Option<bool>,    

    /// draw_copper_wires :: bool (optional)
    draw_copper_wires: Option<bool>,    

    /// draw_logistic_radius_visualization :: bool (optional)
    draw_logistic_radius_visualization: Option<bool>,    

    /// logistics_connection_distance :: float (optional)
    logistics_connection_distance: Option<f32>,    

    /// open_door_trigger_effect :: TriggerEffect (optional)
    open_door_trigger_effect: Option<TriggerEffect>,    

    /// recharging_light :: LightDefinition (optional)
    recharging_light: Option<LightDefinition>,    

    /// robot_limit :: ItemCountType (optional)
    robot_limit: Option<ItemCountType>,    

    /// robot_vertical_acceleration :: float (optional)
    robot_vertical_acceleration: Option<f32>,    

    /// robots_shrink_when_entering_and_exiting :: bool (optional)
    robots_shrink_when_entering_and_exiting: Option<bool>,    

    /// spawn_and_station_shadow_height_offset :: float (optional)
    spawn_and_station_shadow_height_offset: Option<f32>,    

    /// stationing_offset :: vector (optional)
    stationing_offset: Option<Vector2<f32>>,    

}

impl Prototype for Roboport {
    const TYPE: Option<&'static str> = Some("roboport");
}


