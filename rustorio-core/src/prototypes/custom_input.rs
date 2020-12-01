use serde::{Deserialize, Serialize};

use super::Prototype;
use crate::types::*; // TODO: Import only specific types

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CustomInput {
    /// key_sequence :: string
    key_sequence: String,

    /// action :: string (optional)
    action: Option<String>,

    /// alternative_key_sequence :: string (optional)
    alternative_key_sequence: Option<String>,

    /// consuming :: ConsumingType (optional)
    consuming: Option<ConsumingType>,

    /// enabled :: bool (optional)
    enabled: Option<bool>,

    /// enabled_while_in_cutscene :: bool (optional)
    enabled_while_in_cutscene: Option<bool>,

    /// enabled_while_spectating :: bool (optional)
    enabled_while_spectating: Option<bool>,

    /// item_to_create :: string (optional)
    item_to_create: Option<String>,

    /// linked_game_control :: string (optional)
    linked_game_control: Option<String>,
}

impl Prototype for CustomInput {
    const TYPE: Option<&'static str> = Some("custom-input");
}
