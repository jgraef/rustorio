use serde::{Serialize, Deserialize};
        
use super::{Prototype, type_stubs, type_stubs::*};
        
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Animation {
    /// name :: string
    name: String,    

    /// type :: string
    r#type: String,    

    /// animation_speed :: float (optional)
    animation_speed: Option<f32>,    

    /// apply_runtime_tint :: bool (optional)
    apply_runtime_tint: Option<bool>,    

    /// blend_mode :: BlendMode (optional)
    blend_mode: Option<BlendMode>,    

    /// draw_as_shadow :: bool (optional)
    draw_as_shadow: Option<bool>,    

    /// filename :: FileName (optional)
    filename: Option<FileName>,    

    /// flags :: SpriteFlags (optional)
    flags: Option<SpriteFlags>,    

    /// frame_count :: uint32 (optional)
    frame_count: Option<u32>,    

    /// frame_sequence :: AnimationFrameSequence (optional)
    frame_sequence: Option<AnimationFrameSequence>,    

    /// generate_sdf :: bool (optional)
    generate_sdf: Option<bool>,    

    /// height :: SpriteSizeType (optional)
    height: Option<SpriteSizeType>,    

    /// hr_version :: Animation (optional)
    hr_version: Option<type_stubs::Animation>,

    /// layers :: table of Animation (optional)
    layers: Option<Vec<type_stubs::Animation>>,

    /// line_length :: uint32 (optional)
    line_length: Option<u32>,    

    /// load_in_minimal_mode :: bool (optional)
    load_in_minimal_mode: Option<bool>,    

    /// max_advance :: float (optional)
    max_advance: Option<f32>,    

    /// mipmap_count :: uint8 (optional)
    mipmap_count: Option<u8>,    

    /// position :: table of SpriteSizeType (optional)
    position: Option<Vec<SpriteSizeType>>,    

    /// premul_alpha :: bool (optional)
    premul_alpha: Option<bool>,    

    /// priority :: SpritePriority (optional)
    priority: Option<SpritePriority>,    

    /// repeat_count :: uint8 (optional)
    repeat_count: Option<u8>,    

    /// run_mode :: string (optional)
    run_mode: Option<String>,    

    /// scale :: double (optional)
    scale: Option<f64>,    

    /// shift :: vector (optional)
    shift: Option<Vector2<f32>>,    

    /// size :: SpriteSizeType or table of SpriteSizeType (optional)
    size: Option<Todo>,

    /// stripes :: table of Stripe (optional)
    stripes: Option<Vec<Stripe>>,    

    /// tint :: Color (optional)
    tint: Option<Color>,    

    /// width :: SpriteSizeType (optional)
    width: Option<SpriteSizeType>,    

    /// x :: SpriteSizeType (optional)
    x: Option<SpriteSizeType>,    

    /// y :: SpriteSizeType (optional)
    y: Option<SpriteSizeType>,    

}

impl Prototype for Animation {
    const TYPE: Option<&'static str> = Some("animation");
}


