use serde::{Deserialize, Serialize};

use crate::prototypes::{Prototype, Visitor};
use crate::types::*; // TODO: Import only specific types

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MiningDrill {
    /// energy_source :: EnergySource
    energy_source: EnergySource,

    /// energy_usage :: Energy
    energy_usage: Energy,

    /// mining_speed :: double
    mining_speed: f64,

    /// resource_categories :: array of string
    resource_categories: Vec<String>,

    /// resource_searching_radius :: double
    resource_searching_radius: f64,

    /// vector_to_place_result :: vector
    vector_to_place_result: Vector2<f32>,

    /// allowed_effects :: EffectTypeLimitation (optional)
    allowed_effects: Option<EffectTypeLimitation>,

    /// animations :: Animation4Way (optional)
    animations: Option<Animation4Way>,

    /// base_picture :: Sprite4Way (optional)
    base_picture: Option<Sprite4Way>,

    /// base_productivity :: float (optional)
    base_productivity: Option<f32>,

    /// base_render_layer :: RenderLayer (optional)
    base_render_layer: Option<RenderLayer>,

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

    /// graphics_set :: MiningDrillGraphicsSet (optional)
    graphics_set: Option<MiningDrillGraphicsSet>,

    /// input_fluid_box :: FluidBox (optional)
    input_fluid_box: Option<FluidBox>,

    /// module_specification :: ModuleSpecification (optional)
    module_specification: Option<ModuleSpecification>,

    /// monitor_visualization_tint :: Color (optional)
    monitor_visualization_tint: Option<Color>,

    /// output_fluid_box :: FluidBox (optional)
    output_fluid_box: Option<FluidBox>,

    /// radius_visualisation_picture :: Sprite (optional)
    radius_visualisation_picture: Option<Sprite>,

    /// storage_slots :: ItemStackIndex (optional)
    storage_slots: Option<ItemStackIndex>,

    /// wet_mining_graphics_set :: MiningDrillGraphicsSet (optional)
    wet_mining_graphics_set: Option<MiningDrillGraphicsSet>,
}

impl Prototype for MiningDrill {
    const TYPE: Option<&'static str> = Some("mining-drill");
}
