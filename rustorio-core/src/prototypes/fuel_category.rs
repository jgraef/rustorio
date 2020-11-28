use serde::{Serialize, Deserialize};
        
use super::{Prototype, type_stubs::*};
        
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FuelCategory {

}

impl Prototype for FuelCategory {
    const TYPE: Option<&'static str> = Some("fuel-category");
}


