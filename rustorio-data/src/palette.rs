use palette::LinSrgba;

use crate::{FromLuaTable, Error, to_option, to_result};
use mlua::{Value, Table};


pub type Color = LinSrgba<f32>;


impl FromLuaTable for Color {
    fn from_lua_table(table: Table) -> Result<Self, Error> {
        let err = || Error::other("Expected either array or table of RGBA values");

        let r = to_option::<f32>(table.get::<_, Value>(1)?)?;
        let mut c = if let Some(r) = r {
            let g = to_result::<f32, _, _>(table.get::<_, Value>(2)?, err)?;
            let b = to_result::<f32, _, _>(table.get::<_, Value>(3)?, err)?;
            let a = to_result::<f32, _, _>(table.get::<_, Value>(4)?, err)?;
            Color::new(r, g, b, a)
        }
        else {
            let r = to_result::<f32, _, _>(table.get::<_, Value>("r")?, err)?;
            let g = to_result::<f32, _, _>(table.get::<_, Value>("g")?, err)?;
            let b = to_result::<f32, _, _>(table.get::<_, Value>("b")?, err)?;
            let a = to_result::<f32, _, _>(table.get::<_, Value>("a")?, err)?;
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
