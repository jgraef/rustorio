use serde::{Serialize, Deserialize};
        
use super::{Prototype, type_stubs::*};
        
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Armor {
    /// equipment_grid :: string (optional)
    equipment_grid: Option<String>,    

    /// inventory_size_bonus :: ItemStackIndex (optional)
    inventory_size_bonus: Option<ItemStackIndex>,    

    /// resistances :: Resistances (optional)
    resistances: Option<Resistances>,    

}

impl Prototype for Armor {
    const TYPE: Option<&'static str> = Some("armor");
}


