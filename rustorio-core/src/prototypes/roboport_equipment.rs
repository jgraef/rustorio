use serde::{Deserialize, Serialize};

use super::Prototype;
use crate::types::*; // TODO: Import only specific types

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RoboportEquipment {
    /// charge_approach_distance :: float
    charge_approach_distance: f32,

    /// charging_energy :: Energy
    charging_energy: Energy,

    /// construction_radius :: float
    construction_radius: f32,

    /// recharging_animation :: Animation
    recharging_animation: Animation,

    /// spawn_and_station_height :: float
    spawn_and_station_height: f32,

    /// burner :: EnergySource (optional)
    burner: Option<EnergySource>,

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

    /// draw_construction_radius_visualization :: bool (optional)
    draw_construction_radius_visualization: Option<bool>,

    /// draw_logistic_radius_visualization :: bool (optional)
    draw_logistic_radius_visualization: Option<bool>,

    /// power :: Energy (optional)
    power: Option<Energy>,

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

    /// spawn_minimum :: Energy (optional)
    spawn_minimum: Option<Energy>,

    /// stationing_offset :: vector (optional)
    stationing_offset: Option<Vector2<f32>>,
}

impl Prototype for RoboportEquipment {
    const TYPE: Option<&'static str> = Some("roboport-equipment");
}
