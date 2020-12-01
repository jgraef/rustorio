use serde::{Deserialize, Serialize};

use super::Prototype;
use crate::types::*; // TODO: Import only specific types

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SimpleEntityWithOwner {
    /// animations :: AnimationVariations (optional)
    animations: Option<AnimationVariations>,

    /// force_visibility :: ForceCondition (optional)
    force_visibility: Option<ForceCondition>,

    /// picture :: Sprite (optional)
    picture: Option<Sprite>,

    /// pictures :: SpriteVariations (optional)
    pictures: Option<SpriteVariations>,

    /// random_animation_offset :: bool (optional)
    random_animation_offset: Option<bool>,

    /// random_variation_on_create :: bool (optional)
    random_variation_on_create: Option<bool>,

    /// render_layer :: RenderLayer (optional)
    render_layer: Option<RenderLayer>,

    /// secondary_draw_order :: int8 (optional)
    secondary_draw_order: Option<i8>,
}

impl Prototype for SimpleEntityWithOwner {
    const TYPE: Option<&'static str> = Some("simple-entity-with-owner");
}
