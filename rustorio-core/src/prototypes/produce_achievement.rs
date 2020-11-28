use serde::{Serialize, Deserialize};
        
use super::{Prototype, type_stubs::*};
        
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProduceAchievement {
    /// amount :: MaterialAmountType
    amount: MaterialAmountType,    

    /// limited_to_one_game :: bool
    limited_to_one_game: bool,    

    /// fluid_product :: string (optional)
    fluid_product: Option<String>,    

    /// item_product :: string (optional)
    item_product: Option<String>,    

}

impl Prototype for ProduceAchievement {
    const TYPE: Option<&'static str> = Some("produce-achievement");
}


