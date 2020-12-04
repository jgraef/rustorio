use serde::{Deserialize, Serialize};

use crate::prototypes::{Prototype, Visitor};
use crate::types::*; // TODO: Import only specific types

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Corpse {
    /// animation :: RotatedAnimationVariations (optional)
    animation: Option<RotatedAnimationVariations>,

    /// animation_overlay :: RotatedAnimationVariations (optional)
    animation_overlay: Option<RotatedAnimationVariations>,

    /// animation_overlay_final_render_layer :: RenderLayer (optional)
    animation_overlay_final_render_layer: Option<RenderLayer>,

    /// animation_overlay_render_layer :: RenderLayer (optional)
    animation_overlay_render_layer: Option<RenderLayer>,

    /// animation_render_layer :: RenderLayer (optional)
    animation_render_layer: Option<RenderLayer>,

    /// direction_shuffle :: table of tables of uint16 (optional)
    direction_shuffle: Option<Vec<Vec<u16>>>,

    /// dying_speed :: float (optional)
    dying_speed: Option<f32>,

    /// final_render_layer :: RenderLayer (optional)
    final_render_layer: Option<RenderLayer>,

    /// ground_patch :: AnimationVariations (optional)
    ground_patch: Option<AnimationVariations>,

    /// ground_patch_fade_in_delay :: float (optional)
    ground_patch_fade_in_delay: Option<f32>,

    /// ground_patch_fade_in_speed :: float (optional)
    ground_patch_fade_in_speed: Option<f32>,

    /// ground_patch_fade_out_duration :: float (optional)
    ground_patch_fade_out_duration: Option<f32>,

    /// ground_patch_fade_out_start :: float (optional)
    ground_patch_fade_out_start: Option<f32>,

    /// ground_patch_higher :: AnimationVariations (optional)
    ground_patch_higher: Option<AnimationVariations>,

    /// ground_patch_render_layer :: RenderLayer (optional)
    ground_patch_render_layer: Option<RenderLayer>,

    /// remove_on_entity_placement :: bool (optional)
    remove_on_entity_placement: Option<bool>,

    /// remove_on_tile_placement :: bool (optional)
    remove_on_tile_placement: Option<bool>,

    /// shuffle_directions_at_frame :: uint8 (optional)
    shuffle_directions_at_frame: Option<u8>,

    /// splash :: AnimationVariations (optional)
    splash: Option<AnimationVariations>,

    /// splash_render_layer :: RenderLayer (optional)
    splash_render_layer: Option<RenderLayer>,

    /// splash_speed :: float (optional)
    splash_speed: Option<f32>,

    /// time_before_removed :: int32 (optional)
    time_before_removed: Option<i32>,

    /// time_before_shading_off :: int32 (optional)
    time_before_shading_off: Option<i32>,

    /// use_tile_color_for_ground_patch_tint :: bool (optional)
    use_tile_color_for_ground_patch_tint: Option<bool>,
}

impl Prototype for Corpse {
    const TYPE: Option<&'static str> = Some("corpse");
}
