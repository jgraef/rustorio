use serde::{Serialize, Deserialize};
        
use super::{Prototype, type_stubs::*};
        
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Technology {
    /// icons, icon, icon_size (IconSpecification) :: IconSpecification
    icon_spec: IconSpecification,    

    /// unit :: table
    unit: Vec<Todo>,

    /// effects :: table of Modifier (optional)
    effects: Option<Vec<Modifier>>,    

    /// enabled :: bool (optional)
    enabled: Option<bool>,    

    /// expensive :: Technology data or bool (optional)
    expensive: Option<Todo>,

    /// hidden :: bool (optional)
    hidden: Option<bool>,    

    /// max_level :: uint32 or string (optional)
    max_level: Option<Todo>,

    /// normal :: Technology data or bool (optional)
    normal: Option<Todo>,

    /// prerequisites :: table of string (optional)
    prerequisites: Option<Vec<String>>,

    /// upgrade :: bool (optional)
    upgrade: Option<bool>,    

    /// visible_when_disabled :: bool (optional)
    visible_when_disabled: Option<bool>,    

}

impl Prototype for Technology {
    const TYPE: Option<&'static str> = Some("technology");
}


