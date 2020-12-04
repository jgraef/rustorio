use serde::{Deserialize, Serialize};
use mlua::{Value, Table};


use crate::prototypes::{Prototype, Visitor};
use crate::types::{self, *};

#[derive(Clone, Debug, Serialize, Deserialize, rustorio_data_derive::FromLuaTable)]
pub struct Sprite {
    /// name :: string
    name: String,

    /// type :: string
    r#type: String,

    #[lua(flatten)]
    sprite: types::Sprite,
}

impl Prototype for Sprite {
    const TYPE: Option<&'static str> = Some("sprite");
}
