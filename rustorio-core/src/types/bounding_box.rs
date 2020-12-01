//! Type for bounding boxes. See [1].
//!
//! [1] https://wiki.factorio.com/Types/BoundingBox
//!
//! # TODO
//!
//!  - Write tests for serialization
//!

use nalgebra::Point2;
use mlua::{Value, Table};
use serde::{Serialize, Deserialize};

use rustorio_data::{FromLuaTable, Error, to_result, to_option};


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BoundingBox {
    pub top_left: Point2<f32>,
    pub bottom_right: Point2<f32>,
    pub orientation: Option<f32>,
}

impl FromLuaTable for BoundingBox {
    fn from_lua_table(table: Table) -> Result<Self, Error> {
        let err = || Error::other("2 points defining a bounding-box");
        let top_left = to_result::<Point2<f32>, _, _>(table.get::<_, Value>(1)?, err)?;
        let bottom_right = to_result::<Point2<f32>, _, _>(table.get::<_, Value>(2)?, err)?;
        let orientation = to_option::<f32>(table.get::<_, Value>(3)?)?;

        Ok(BoundingBox {
            top_left,
            bottom_right,
            orientation
        })
    }
}
