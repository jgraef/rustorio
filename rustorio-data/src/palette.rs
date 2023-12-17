use mlua::{
    Table,
    Value,
};
use palette::LinSrgba;

use crate::{
    to_option,
    Error,
    FromLuaTable,
};

pub type Color = LinSrgba<f32>;

impl FromLuaTable for Color {
    fn from_lua_table(table: Table) -> Result<Self, Error> {
        let r = to_option::<f32>(table.get::<_, Value>(1)?)?;
        let mut c = if let Some(r) = r {
            let g = to_option(table.get::<_, Value>(2)?)?.unwrap_or_default();
            let b = to_option(table.get::<_, Value>(3)?)?.unwrap_or_default();
            let a = to_option(table.get::<_, Value>(4)?)?.unwrap_or_default();
            Color::new(r, g, b, a)
        }
        else {
            let r = to_option(table.get::<_, Value>("r")?)?.unwrap_or_default();
            let g = to_option(table.get::<_, Value>("g")?)?.unwrap_or_default();
            let b = to_option(table.get::<_, Value>("b")?)?.unwrap_or_default();
            let a = to_option(table.get::<_, Value>("a")?)?.unwrap_or_default();
            Color::new(r, g, b, a)
        };

        if c.color.red > 1. || c.color.green > 1. || c.color.blue > 1. || c.alpha > 1. {
            c.color.red /= 255.;
            c.color.green /= 255.;
            c.color.blue /= 255.;
            c.alpha /= 255.;
        }

        Ok(c)
    }
}
