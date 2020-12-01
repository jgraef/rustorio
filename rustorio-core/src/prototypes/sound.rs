use serde::{Deserialize, Serialize};

use super::Prototype;
use crate::types::*; // TODO: Import only specific types

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Sound {
    /// name :: string
    name: String,

    /// type :: string
    r#type: String,

    /// aggregation :: table (optional)
    aggregation: Option<Vec<Todo>>,

    /// allow_random_repeat :: bool (optional)
    allow_random_repeat: Option<bool>,

    /// audible_distance_modifier :: double (optional)
    audible_distance_modifier: Option<f64>,

    /// category :: string (optional)
    category: Option<String>,

    /// filename :: FileName (optional)
    filename: Option<FileName>,

    /// max_speed :: float (optional)
    max_speed: Option<f32>,

    /// min_speed :: float (optional)
    min_speed: Option<f32>,

    /// preload :: bool (optional)
    preload: Option<bool>,

    /// speed :: float (optional)
    speed: Option<f32>,

    /// variations :: table (array) of tables (optional)
    variations: Option<Vec<Todo>>,

    /// volume :: float (optional)
    volume: Option<f32>,
}

impl Prototype for Sound {
    const TYPE: Option<&'static str> = Some("sound");
}
