use serde::{Deserialize, Serialize};

use crate::prototypes::{Prototype, Visitor};
use crate::types::*; // TODO: Import only specific types

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ResourceEntity {
    /// stage_counts :: table of uint32
    stage_counts: Vec<u32>,

    /// stages :: AnimationVariations
    stages: AnimationVariations,

    /// category :: string (optional)
    category: Option<String>,

    /// effect_animation_period :: float (optional)
    effect_animation_period: Option<f32>,

    /// effect_animation_period_deviation :: float (optional)
    effect_animation_period_deviation: Option<f32>,

    /// effect_darkness_multiplier :: float (optional)
    effect_darkness_multiplier: Option<f32>,

    /// highlight :: bool (optional)
    highlight: Option<bool>,

    /// infinite :: bool (optional)
    infinite: Option<bool>,

    /// infinite_depletion_amount :: uint32 (optional)
    infinite_depletion_amount: Option<u32>,

    /// map_grid :: bool (optional)
    map_grid: Option<bool>,

    /// max_effect_alpha :: float (optional)
    max_effect_alpha: Option<f32>,

    /// min_effect_alpha :: float (optional)
    min_effect_alpha: Option<f32>,

    /// minimum :: uint32 (optional)
    minimum: Option<u32>,

    /// mining_visualisation_tint :: Color (optional)
    mining_visualisation_tint: Option<Color>,

    /// normal :: uint32 (optional)
    normal: Option<u32>,

    /// randomize_visual_position :: bool (optional)
    randomize_visual_position: Option<bool>,

    /// resource_patch_search_radius :: uint32 (optional)
    resource_patch_search_radius: Option<u32>,

    /// stages_effect :: AnimationVariations (optional)
    stages_effect: Option<AnimationVariations>,

    /// tree_removal_max_distance :: double (optional)
    tree_removal_max_distance: Option<f64>,

    /// tree_removal_probability :: double (optional)
    tree_removal_probability: Option<f64>,

    /// walking_sound :: Sound (optional)
    walking_sound: Option<Sound>,
}

impl Prototype for ResourceEntity {
    const TYPE: Option<&'static str> = Some("resource");
}
