use serde::{Deserialize, Serialize};

use super::Prototype;
use crate::types::*; // TODO: Import only specific types

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SpeechBubble {
    /// style :: string
    style: String,

    /// fade_in_out_ticks :: uint32 (optional)
    fade_in_out_ticks: Option<u32>,

    /// wrapper_flow_style :: string (optional)
    wrapper_flow_style: Option<String>,

    /// y_offset :: double (optional)
    y_offset: Option<f64>,
}

impl Prototype for SpeechBubble {
    const TYPE: Option<&'static str> = Some("speech-bubble");
}
