use serde::{Serialize, Deserialize};
        
use super::{Prototype, type_stubs::*};
        
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Decorative {
    /// pictures :: SpriteVariations
    pictures: SpriteVariations,    

    /// autoplace :: AutoplaceSpecification (optional)
    autoplace: Option<AutoplaceSpecification>,    

    /// collision_box :: BoundingBox (optional)
    collision_box: Option<BoundingBox>,    

    /// collision_mask :: CollisionMask (optional)
    collision_mask: Option<CollisionMask>,    

    /// grows_through_rail_path :: bool (optional)
    grows_through_rail_path: Option<bool>,    

    /// render_layer :: RenderLayer (optional)
    render_layer: Option<RenderLayer>,    

    /// tile_layer :: uint8 (optional)
    tile_layer: Option<u8>,    

    /// trigger_effect :: TriggerEffect (optional)
    trigger_effect: Option<TriggerEffect>,    

    /// walking_sound :: Sound (optional)
    walking_sound: Option<Sound>,    

}

impl Prototype for Decorative {
    const TYPE: Option<&'static str> = Some("optimized-decorative");
}


