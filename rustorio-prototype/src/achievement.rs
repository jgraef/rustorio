#[cfg(feature = "lua-api")]
use rustorio_lua_api::FromLuaTable;
#[cfg(feature = "serde")]
use serde::{
    Deserialize,
    Serialize,
};

use super::{
    Inherits,
    PrototypeBase,
};
use crate::types::IconSpecification;

#[derive(Clone, Debug)]
#[cfg_attr(feature = "lua-api", derive(FromLuaTable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AchievementPrototype {
    #[cfg_attr(feature = "lua-api", lua(flatten))]
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub parent: PrototypeBase,

    #[cfg_attr(feature = "lua-api", lua(flatten))]
    pub icon_spec: IconSpecification,

    #[cfg_attr(feature = "lua-api", lua(default_with = "true"))]
    pub allowed_without_fight: bool,

    #[cfg_attr(feature = "lua-api", lua(default))]
    pub hidden: bool,

    #[cfg_attr(feature = "lua-api", lua(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub steam_stats_name: Option<String>,
}

impl Inherits for AchievementPrototype {
    type Parent = PrototypeBase;

    fn parent(&self) -> &PrototypeBase {
        &self.parent
    }
}
