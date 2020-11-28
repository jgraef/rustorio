use serde::{Serialize, Deserialize};
        
use super::{Prototype, type_stubs::*};
        
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CargoWagon {
    /// inventory_size :: ItemStackIndex
    inventory_size: ItemStackIndex,    

}

impl Prototype for CargoWagon {
    const TYPE: Option<&'static str> = Some("cargo-wagon");
}


