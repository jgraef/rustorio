use serde::{Serialize, Deserialize};
        
use super::{Prototype, type_stubs::*};
        
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Furnace {
    /// result_inventory_size :: ItemStackIndex
    result_inventory_size: ItemStackIndex,    

    /// source_inventory_size :: ItemStackIndex
    source_inventory_size: ItemStackIndex,    

}

impl Prototype for Furnace {
    const TYPE: Option<&'static str> = Some("furnace");
}


