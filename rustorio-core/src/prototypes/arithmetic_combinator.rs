use serde::{Deserialize, Serialize};

use super::Prototype;
use crate::types::*; // TODO: Import only specific types

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ArithmeticCombinator {
    /// and_symbol_sprites :: Sprite4Way
    and_symbol_sprites: Sprite4Way,

    /// divide_symbol_sprites :: Sprite4Way
    divide_symbol_sprites: Sprite4Way,

    /// left_shift_symbol_sprites :: Sprite4Way
    left_shift_symbol_sprites: Sprite4Way,

    /// minus_symbol_sprites :: Sprite4Way
    minus_symbol_sprites: Sprite4Way,

    /// modulo_symbol_sprites :: Sprite4Way
    modulo_symbol_sprites: Sprite4Way,

    /// multiply_symbol_sprites :: Sprite4Way
    multiply_symbol_sprites: Sprite4Way,

    /// or_symbol_sprites :: Sprite4Way
    or_symbol_sprites: Sprite4Way,

    /// plus_symbol_sprites :: Sprite4Way
    plus_symbol_sprites: Sprite4Way,

    /// power_symbol_sprites :: Sprite4Way
    power_symbol_sprites: Sprite4Way,

    /// right_shift_symbol_sprites :: Sprite4Way
    right_shift_symbol_sprites: Sprite4Way,

    /// xor_symbol_sprites :: Sprite4Way
    xor_symbol_sprites: Sprite4Way,
}

impl Prototype for ArithmeticCombinator {
    const TYPE: Option<&'static str> = Some("arithmetic-combinator");
}
