use serde::{Deserialize, Serialize};

use crate::prototypes::{Prototype, Visitor};
use crate::types::*; // TODO: Import only specific types

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DontCraftManuallyAchievement {
    /// amount :: MaterialAmountType
    amount: MaterialAmountType,
}

impl Prototype for DontCraftManuallyAchievement {
    const TYPE: Option<&'static str> = Some("dont-craft-manually-achievement");
}
