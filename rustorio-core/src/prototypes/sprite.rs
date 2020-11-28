use serde::{Serialize, Deserialize};
        
use super::{Prototype, type_stubs, type_stubs::*};
        
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Sprite {
    /// filename :: FileName
    filename: FileName,    

    /// name :: string
    name: String,    

    /// type :: string
    r#type: String,    

    /// apply_runtime_tint :: bool (optional)
    apply_runtime_tint: Option<bool>,    

    /// blend_mode :: BlendMode (optional)
    blend_mode: Option<BlendMode>,    

    /// draw_as_shadow :: bool (optional)
    draw_as_shadow: Option<bool>,    

    /// flags :: SpriteFlags (optional)
    flags: Option<SpriteFlags>,    

    /// generate_sdf :: bool (optional)
    generate_sdf: Option<bool>,    

    /// height :: SpriteSizeType (optional)
    height: Option<SpriteSizeType>,    

    /// hr_version :: Sprite (optional)
    hr_version: Option<type_stubs::Sprite>,

    /// layers :: table of Sprite (optional)
    layers: Option<Vec<type_stubs::Sprite>>,

    /// load_in_minimal_mode :: bool (optional)
    load_in_minimal_mode: Option<bool>,    

    /// mipmap_count :: uint8 (optional)
    mipmap_count: Option<u8>,    

    /// position :: table of SpriteSizeType (optional)
    position: Option<Vec<SpriteSizeType>>,    

    /// premul_alpha :: bool (optional)
    premul_alpha: Option<bool>,    

    /// priority :: SpritePriority (optional)
    priority: Option<SpritePriority>,    

    /// scale :: double (optional)
    scale: Option<f64>,    

    /// shift :: vector (optional)
    shift: Option<Vector2<f32>>,    

    /// size :: SpriteSizeType or table of SpriteSizeType (optional)
    size: Option<Todo>,

    /// slice :: SpriteSizeType (optional)
    slice: Option<SpriteSizeType>,    

    /// slice_x :: SpriteSizeType (optional)
    slice_x: Option<SpriteSizeType>,    

    /// slice_y :: SpriteSizeType (optional)
    slice_y: Option<SpriteSizeType>,    

    /// tint :: Color (optional)
    tint: Option<Color>,    

    /// width :: SpriteSizeType (optional)
    width: Option<SpriteSizeType>,    

    /// x :: SpriteSizeType (optional)
    x: Option<SpriteSizeType>,    

    /// y :: SpriteSizeType (optional)
    y: Option<SpriteSizeType>,    

}

impl Prototype for Sprite {
    const TYPE: Option<&'static str> = Some("sprite");
}


