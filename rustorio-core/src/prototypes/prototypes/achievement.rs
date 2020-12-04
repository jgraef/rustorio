use serde::{Deserialize, Serialize};

use crate::types::*; // TODO: Import only specific types

use rustorio_data_derive::{FromLuaTable, Prototype};


#[derive(Clone, Debug, Serialize, Deserialize, FromLuaTable, Prototype)]
#[prototype(inherits="super::prototype_base::PrototypeBase")]
pub struct Achievement {
    /// icons, icon, icon_size (IconSpecification) :: IconSpecification
    #[lua(flatten)]
    icon_spec: IconSpecification,

    /// allowed_without_fight :: bool (optional)
    #[lua(default_with="true")]
    allowed_without_fight: bool,

    /// hidden :: bool (optional)
    #[lua(default)]
    hidden: bool,

    /// steam_stats_name :: string (optional)
    #[lua(default)]
    steam_stats_name: Option<String>,
}
