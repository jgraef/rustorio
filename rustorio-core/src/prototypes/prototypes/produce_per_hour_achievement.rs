use serde::{Deserialize, Serialize};

use crate::prototypes::{Prototype, Visitor};
use crate::types::*; // TODO: Import only specific types

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProducePerHourAchievement {
    /// amount :: MaterialAmountType
    amount: MaterialAmountType,

    /// fluid_product :: string (optional)
    fluid_product: Option<String>,

    /// item_product :: string (optional)
    item_product: Option<String>,
}

impl Prototype for ProducePerHourAchievement {
    const TYPE: Option<&'static str> = Some("produce-per-hour-achievement");
}
