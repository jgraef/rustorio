use serde::{Deserialize, Serialize};

use crate::prototypes::{Prototype, Visitor};
use crate::types::*; // TODO: Import only specific types

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ItemGroup {
    /// icons, icon, icon_size (IconSpecification) :: IconSpecification
    icon_spec: IconSpecification,

    /// order_in_recipe :: Order (optional)
    order_in_recipe: Option<Order>,
}

impl Prototype for ItemGroup {
    const TYPE: Option<&'static str> = Some("item-group");
}
