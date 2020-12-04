use serde::{Deserialize, Serialize};

use crate::types::*; // TODO: Import only specific types

use rustorio_data_derive::{FromLuaTable, Prototype};


#[derive(Clone, Debug, Serialize, Deserialize, FromLuaTable, Prototype)]
#[prototype(inherits="super::entity_with_health::EntityWithHealth")]
pub struct Accumulator {
    /// charge_cooldown :: uint16
    charge_cooldown: u16,

    /// discharge_cooldown :: uint16
    discharge_cooldown: u16,

    /// energy_source :: EnergySource
    energy_source: EnergySource,

    /// picture :: Sprite
    picture: Sprite,

    /// charge_animation :: Animation (optional)
    #[lua(default)]
    charge_animation: Option<Animation>,

    /// charge_light :: LightDefinition (optional)
    #[lua(default)]
    charge_light: Option<LightDefinition>,

    /// circuit_connector_sprites :: CircuitConnectorSprites (optional)
    #[lua(default)]
    circuit_connector_sprites: Option<CircuitConnectorSprites>,

    /// circuit_wire_connection_point :: WireConnectionPoint (optional)
    #[lua(default)]
    circuit_wire_connection_point: Option<WireConnectionPoint>,

    /// circuit_wire_max_distance :: double (optional)
    #[lua(default)]
    circuit_wire_max_distance: Option<f64>,

    /// default_output_signal :: SignalIDConnector (optional)
    #[lua(default)]
    default_output_signal: Option<SignalIDConnector>,

    /// discharge_animation :: Animation (optional)
    #[lua(default)]
    discharge_animation: Option<Animation>,

    /// discharge_light :: LightDefinition (optional)
    #[lua(default)]
    discharge_light: Option<LightDefinition>,

    /// draw_circuit_wires :: bool (optional)
    #[lua(default)]
    draw_circuit_wires: Option<bool>,

    /// draw_copper_wires :: bool (optional)
    #[lua(default)]
    draw_copper_wires: Option<bool>,
}

