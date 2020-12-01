use serde::{Deserialize, Serialize};

use super::Prototype;
use crate::types::*; // TODO: Import only specific types

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SimpleEntity {
    /// animations :: AnimationVariations (optional)
    animations: Option<AnimationVariations>,

    /// count_as_rock_for_filtered_deconstruction :: bool (optional)
    count_as_rock_for_filtered_deconstruction: Option<bool>,

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

impl Prototype for SimpleEntity {
    const TYPE: Option<&'static str> = Some("simple-entity");
}
