use serde::{Serialize, Deserialize};
        
use super::{Prototype, type_stubs::*};
        
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GodController {
    /// inventory_size :: ItemStackIndex
    inventory_size: ItemStackIndex,    

    /// item_pickup_distance :: double
    item_pickup_distance: f64,    

    /// loot_pickup_distance :: double
    loot_pickup_distance: f64,    

    /// mining_speed :: double
    mining_speed: f64,    

    /// movement_speed :: double
    movement_speed: f64,    

    /// name :: string
    name: String,    

    /// type :: string
    r#type: String,    

    /// crafting_categories :: table of string (optional)
    crafting_categories: Option<Vec<String>>,

    /// mining_categories :: table of string (optional)
    mining_categories: Option<Vec<String>>,

}

impl Prototype for GodController {
    const TYPE: Option<&'static str> = Some("god-controller");
}


