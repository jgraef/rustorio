use serde::{Serialize, Deserialize};
        
use super::{Prototype, type_stubs::*};
        
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ItemWithInventory {
    /// inventory_size :: ItemStackIndex
    inventory_size: ItemStackIndex,    

    /// extends_inventory_by_default :: bool (optional)
    extends_inventory_by_default: Option<bool>,    

    /// filter_message_key :: string (optional)
    filter_message_key: Option<String>,    

    /// filter_mode :: string (optional)
    filter_mode: Option<String>,    

    /// insertion_priority_mode :: string (optional)
    insertion_priority_mode: Option<String>,    

    /// item_filters :: table of string (optional)
    item_filters: Option<Vec<String>>,

    /// item_group_filters :: table of string (optional)
    item_group_filters: Option<Vec<String>>,

    /// item_subgroup_filters :: table of string (optional)
    item_subgroup_filters: Option<Vec<String>>,

}

impl Prototype for ItemWithInventory {
    const TYPE: Option<&'static str> = Some("item-with-inventory");
}


