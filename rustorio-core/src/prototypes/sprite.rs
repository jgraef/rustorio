use serde::{Deserialize, Serialize};
use mlua::{Value, Table};

use rustorio_data::{
    nalgebra::vec_from_fields,
    FromLuaTable, FromLuaValue, Error, to_option
};

use super::Prototype;
use crate::types::{self, *};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Sprite {
    /// filename :: FileName
    filename: FileName,

    /// name :: string
    name: String,

    /// type :: string
    r#type: String,

    /// apply_runtime_tint :: bool (optional)
    apply_runtime_tint: bool,

    /// blend_mode :: BlendMode (optional)
    blend_mode: BlendMode,

    /// draw_as_shadow :: bool (optional)
    draw_as_shadow: bool,

    /// flags :: SpriteFlags (optional)
    flags: Option<SpriteFlags>,

    /// generate_sdf :: bool (optional)
    generate_sdf: bool,

    /// hr_version :: Sprite (optional)
    hr_version: Option<types::Sprite>,

    /// layers :: table of Sprite (optional)
    layers: Option<Vec<types::Sprite>>,

    /// load_in_minimal_mode :: bool (optional)
    load_in_minimal_mode: bool,

    /// mipmap_count :: uint8 (optional)
    mipmap_count: u8,

    /// position :: table of SpriteSizeType (optional)
    position: Point2<f32>,

    /// premul_alpha :: bool (optional)
    premul_alpha: bool,

    /// priority :: SpritePriority (optional)
    priority: SpritePriority,

    /// scale :: double (optional)
    scale: f64,

    /// shift :: vector (optional)
    shift: Vector2<f32>,

    /// size :: SpriteSizeType or table of SpriteSizeType (optional)
    /// width :: SpriteSizeType (optional)
    /// height :: SpriteSizeType (optional)
    size: Vector2<f32>,

    /// Either one needs to be present, maybe we can handle this with an flattened, untagged enum
    dice: Option<Vector2<f32>>,

    /// tint :: Color (optional)
    tint: Color,
}

impl Prototype for Sprite {
    const TYPE: Option<&'static str> = Some("sprite");
}

impl FromLuaTable for Sprite {
    fn from_lua_table(table: Table) -> Result<Self, Error> {
        let position = vec_from_fields(&table, "position", "x", "y")?;
        let size = vec_from_fields(&table, "size", "width", "height")?;
        let dice = if let Ok(v) = vec_from_fields(&table, "dice", "dice_x", "dice_y") {
            Some(v)
        }
        else {
            vec_from_fields(&table, "slice", "slice_x", "slice_y").ok()
        };

        Ok(Sprite {
            filename: FromLuaValue::from_lua_value(table.get::<_, Value>("filename")?)?,
            name: FromLuaValue::from_lua_value(table.get::<_, Value>("name")?)?,
            r#type: FromLuaValue::from_lua_value(table.get::<_, Value>("type")?)?,
            apply_runtime_tint: to_option(table.get::<_, Value>("apply_runtime_tint")?)?.unwrap_or_default(),
            blend_mode: to_option(table.get::<_, Value>("blend_mode")?)?.unwrap_or_default(),
            draw_as_shadow: to_option(table.get::<_, Value>("draw_as_shadow")?)?.unwrap_or_default(),
            flags: to_option(table.get::<_, Value>("flags")?)?,
            generate_sdf: to_option(table.get::<_, Value>("generate_sdf")?)?.unwrap_or_default(),
            hr_version: to_option(table.get::<_, Value>("hr_version")?)?,
            layers: to_option(table.get::<_, Value>("layers")?)?,
            load_in_minimal_mode: to_option(table.get::<_, Value>("load_in_minimal_mode")?)?.unwrap_or_default(),
            mipmap_count: to_option(table.get::<_, Value>("mipmap_count")?)?.unwrap_or_default(),
            position: Point2::from(position),
            premul_alpha: to_option(table.get::<_, Value>("premul_alpha")?)?.unwrap_or(true),
            priority: to_option(table.get::<_, Value>("priority")?)?.unwrap_or_default(),
            scale: to_option(table.get::<_, Value>("scale")?)?.unwrap_or(1.),
            shift: to_option(table.get::<_, Value>("shift")?)?.unwrap_or_default(),
            size,
            dice,
            tint: to_option(table.get::<_, Value>("tint")?)?.unwrap_or_else(|| Color::new(1., 1., 1., 1.)),
        })
    }
}