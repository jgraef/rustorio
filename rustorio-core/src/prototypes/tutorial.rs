use serde::{Serialize, Deserialize};
        
use super::{Prototype, type_stubs::*};
        
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Tutorial {
    /// icons, icon, icon_size (IconSpecification) :: IconSpecification
    icon_spec: IconSpecification,    

    /// scenario :: string
    scenario: String,    

    /// trigger :: table
    trigger: Vec<Todo>,

    /// dependencies :: table of string (optional)
    dependencies: Option<Vec<String>>,

    /// locked_when_dependencies_not_completed :: bool (optional)
    locked_when_dependencies_not_completed: Option<bool>,    

    /// related_items :: table of string (optional)
    related_items: Option<Vec<String>>,

    /// unlocked :: bool (optional)
    unlocked: Option<bool>,    

}

impl Prototype for Tutorial {
    const TYPE: Option<&'static str> = Some("tutorial");
}


