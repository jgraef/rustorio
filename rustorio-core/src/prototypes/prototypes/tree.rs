use serde::{Deserialize, Serialize};

use crate::prototypes::{Prototype, Visitor};
use crate::types::*; // TODO: Import only specific types

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Tree {
    /// colors :: table of Color (optional)
    colors: Option<Vec<Color>>,

    /// darkness_of_burnt_tree :: float (optional)
    darkness_of_burnt_tree: Option<f32>,

    /// pictures :: SpriteVariations (optional)
    pictures: Option<SpriteVariations>,

    /// variation_weights :: table of double (optional)
    variation_weights: Option<Vec<f64>>,

    /// variations :: table of TreePrototypeVariation (optional)
    variations: Option<Vec<TreePrototypeVariation>>,
}

impl Prototype for Tree {
    const TYPE: Option<&'static str> = Some("tree");
}
