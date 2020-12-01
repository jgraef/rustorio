use serde::{Deserialize, Serialize};

use super::Prototype;
use crate::types::*; // TODO: Import only specific types

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AmbientSound {
    /// name :: string
    name: String,

    /// sound :: Sound
    sound: Sound,

    /// track_type :: string
    track_type: String,

    /// type :: string
    r#type: String,

    /// weight :: double (optional)
    weight: Option<f64>,
}

impl Prototype for AmbientSound {
    const TYPE: Option<&'static str> = Some("ambient-sound");
}
