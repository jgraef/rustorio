use rustorio_data_derive::FromLuaTable;
use serde::{
    Deserialize,
    Serialize,
};

use super::{
    Inherits,
    PrototypeBase,
};
use crate::types::IconSpecification;

#[derive(Clone, Debug, Serialize, Deserialize, FromLuaTable)]
pub struct AchievementPrototype {
    #[lua(flatten)]
    #[serde(flatten)]
    parent: PrototypeBase,

    #[lua(flatten)]
    icon_spec: IconSpecification,

    #[lua(default_with = "true")]
    allowed_without_fight: bool,

    #[lua(default)]
    hidden: bool,

    #[lua(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    steam_stats_name: Option<String>,
}

impl Inherits for AchievementPrototype {
    type Parent = PrototypeBase;

    fn parent(&self) -> &PrototypeBase {
        &self.parent
    }
}
