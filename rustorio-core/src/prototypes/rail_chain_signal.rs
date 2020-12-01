use serde::{Deserialize, Serialize};

use super::Prototype;
use crate::types::*; // TODO: Import only specific types

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RailChainSignal {
    /// selection_box_offsets :: table of vector
    selection_box_offsets: Vec<Vector2<f32>>,

    /// blue_light :: LightDefinition (optional)
    blue_light: Option<LightDefinition>,

    /// default_blue_output_signal :: SignalIDConnector (optional)
    default_blue_output_signal: Option<SignalIDConnector>,
}

impl Prototype for RailChainSignal {
    const TYPE: Option<&'static str> = Some("rail-chain-signal");
}
