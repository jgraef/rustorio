use serde::{Deserialize, Serialize};

use super::Prototype;
use crate::types::*; // TODO: Import only specific types

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeciderCombinator {
    /// equal_symbol_sprites :: Sprite4Way
    equal_symbol_sprites: Sprite4Way,

    /// greater_or_equal_symbol_sprites :: Sprite4Way
    greater_or_equal_symbol_sprites: Sprite4Way,

    /// greater_symbol_sprites :: Sprite4Way
    greater_symbol_sprites: Sprite4Way,

    /// less_or_equal_symbol_sprites :: Sprite4Way
    less_or_equal_symbol_sprites: Sprite4Way,

    /// less_symbol_sprites :: Sprite4Way
    less_symbol_sprites: Sprite4Way,

    /// not_equal_symbol_sprites :: Sprite4Way
    not_equal_symbol_sprites: Sprite4Way,
}

impl Prototype for DeciderCombinator {
    const TYPE: Option<&'static str> = Some("decider-combinator");
}
