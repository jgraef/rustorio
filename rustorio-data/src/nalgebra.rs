use mlua::{
    Table,
    Value,
};
use nalgebra::{
    Point2,
    Point3,
    Scalar,
    Vector2,
    Vector3,
};

use crate::{
    to_option,
    to_result,
    Error,
    FromLuaTable,
    FromLuaValue,
};

impl<T> FromLuaTable for Vector2<T>
where
    T: Scalar + FromLuaValue,
{
    fn from_lua_table(table: Table) -> Result<Self, Error> {
        let err = || Error::other("Expected either `x` and `y` or array of length 2");

        let x = to_option::<T>(table.get::<_, Value>("x")?)?;
        let y = to_option::<T>(table.get::<_, Value>("y")?)?;

        if let (Some(x), Some(y)) = (x, y) {
            Ok(Vector2::new(x, y))
        }
        else {
            let x = to_result::<T, _, _>(table.get::<_, Value>(1)?, err)?;
            let y = to_result::<T, _, _>(table.get::<_, Value>(2)?, err)?;
            Ok(Vector2::new(x, y))
        }
    }
}

impl<T> FromLuaTable for Point2<T>
where
    T: Scalar + FromLuaValue,
{
    fn from_lua_table(table: Table) -> Result<Self, Error> {
        Ok(Point2::from(Vector2::from_lua_table(table)?))
    }
}

impl<T> FromLuaTable for Vector3<T>
where
    T: Scalar + FromLuaValue,
{
    fn from_lua_table(table: Table) -> Result<Self, Error> {
        let err = || Error::other("Expected either `x`, `y`, `z` or array of length 3");

        let x = to_option::<T>(table.get::<_, Value>("x")?)?;
        let y = to_option::<T>(table.get::<_, Value>("y")?)?;
        let z = to_option::<T>(table.get::<_, Value>("z")?)?;

        if let (Some(x), Some(y), Some(z)) = (x, y, z) {
            Ok(Vector3::new(x, y, z))
        }
        else {
            let x = to_result::<T, _, _>(table.get::<_, Value>(1)?, err)?;
            let y = to_result::<T, _, _>(table.get::<_, Value>(2)?, err)?;
            let z = to_result::<T, _, _>(table.get::<_, Value>(3)?, err)?;
            Ok(Vector3::new(x, y, z))
        }
    }
}

impl<T> FromLuaTable for Point3<T>
where
    T: Scalar + FromLuaValue,
{
    fn from_lua_table(table: Table) -> Result<Self, Error> {
        Ok(Point3::from(Vector3::from_lua_table(table)?))
    }
}
