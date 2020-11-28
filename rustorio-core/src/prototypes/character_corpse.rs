use serde::{Serialize, Deserialize};
        
use super::{Prototype, type_stubs::*};
        
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CharacterCorpse {
    /// time_to_live :: uint32
    time_to_live: u32,    

    /// armor_picture_mapping :: table (optional)
    armor_picture_mapping: Option<Vec<Todo>>,

    /// picture :: Animation (optional)
    picture: Option<Animation>,    

    /// pictures :: AnimationVariations (optional)
    pictures: Option<AnimationVariations>,    

    /// render_layer :: RenderLayer (optional)
    render_layer: Option<RenderLayer>,    

}

impl Prototype for CharacterCorpse {
    const TYPE: Option<&'static str> = Some("character-corpse");
}


