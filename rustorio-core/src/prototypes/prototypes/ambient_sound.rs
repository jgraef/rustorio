use serde::{Deserialize, Serialize};

use crate::types::*; // TODO: Import only specific types

use rustorio_data_derive::{FromLuaTable, Prototype};


#[derive(Clone, Debug, Serialize, Deserialize, FromLuaTable, Prototype)]
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
