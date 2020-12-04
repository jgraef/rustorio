use serde::{Deserialize, Serialize};

use crate::prototypes::{Prototype, Visitor};
use crate::types::*; // TODO: Import only specific types

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Item {
    /// icons, icon,  icon_size (IconSpecification) :: IconSpecification
    icon: IconSpecification,

    /// stack_size :: ItemCountType
    stack_size: ItemCountType,

    /// burnt_result :: string (optional)
    burnt_result: Option<String>,

    /// dark_background_icons, dark_background_icon,  icon_size (IconSpecification) :: IconSpecification (optional)
    dark_background_icon: Option<IconSpecification>,

    /// default_request_amount :: ItemCountType (optional)
    default_request_amount: Option<ItemCountType>,

    /// flags :: ItemPrototypeFlags (optional)
    flags: Option<ItemPrototypeFlags>,

    /// fuel_acceleration_multiplier :: double (optional)
    fuel_acceleration_multiplier: Option<f64>,

    /// fuel_category :: string (optional)
    fuel_category: Option<String>,

    /// fuel_emissions_multiplier :: double (optional)
    fuel_emissions_multiplier: Option<f64>,

    /// fuel_glow_color :: Color (optional)
    fuel_glow_color: Option<Color>,

    /// fuel_top_speed_multiplier :: double (optional)
    fuel_top_speed_multiplier: Option<f64>,

    /// fuel_value :: Energy (optional)
    fuel_value: Option<Energy>,

    /// pictures :: SpriteVariations (optional)
    pictures: Option<SpriteVariations>,

    /// place_as_tile :: PlaceAsTile (optional)
    place_as_tile: Option<PlaceAsTile>,

    /// place_result :: string (optional)
    place_result: Option<String>,

    /// placed_as_equipment_result :: string (optional)
    placed_as_equipment_result: Option<String>,

    /// rocket_launch_product :: ItemProductPrototype (optional)
    rocket_launch_product: Option<ItemProductPrototype>,

    /// rocket_launch_products :: table (array) of ItemProductPrototype (optional)
    rocket_launch_products: Option<Vec<ItemProductPrototype>>,

    /// subgroup :: string (optional)
    subgroup: Option<String>,

    /// wire_count :: ItemCountType (optional)
    wire_count: Option<ItemCountType>,
}

impl Prototype for Item {
    const TYPE: Option<&'static str> = Some("item");
}
