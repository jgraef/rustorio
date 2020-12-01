use serde::{Deserialize, Serialize};

use super::Prototype;
use crate::types::*; // TODO: Import only specific types

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
