use serde::{Serialize, Deserialize};
        
use super::{Prototype, type_stubs::*};
        
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Achievement {
    /// icons, icon, icon_size (IconSpecification) :: IconSpecification
    icon_spec: IconSpecification,    

    /// allowed_without_fight :: bool (optional)
    allowed_without_fight: Option<bool>,    

    /// hidden :: bool (optional)
    hidden: Option<bool>,    

    /// steam_stats_name :: string (optional)
    steam_stats_name: Option<String>,    

}

impl Prototype for Achievement {
    const TYPE: Option<&'static str> = Some("achievement");
}


