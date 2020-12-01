use serde::{Deserialize, Serialize};

use super::Prototype;
use crate::types::*; // TODO: Import only specific types

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Equipment {
    /// categories :: table of string
    categories: Vec<String>,

    /// energy_source :: EnergySource
    energy_source: EnergySource,

    /// shape :: EquipmentShape
    shape: EquipmentShape,

    /// sprite :: Sprite
    sprite: Sprite,

    /// ability_icon :: Sprite (optional)
    ability_icon: Option<Sprite>,

    /// background_border_color :: Color (optional)
    background_border_color: Option<Color>,

    /// background_color :: Color (optional)
    background_color: Option<Color>,

    /// grabbed_background_color :: Color (optional)
    grabbed_background_color: Option<Color>,

    /// take_result :: string (optional)
    take_result: Option<String>,
}

impl Prototype for Equipment {
    const TYPE: Option<&'static str> = Some("None");
}
