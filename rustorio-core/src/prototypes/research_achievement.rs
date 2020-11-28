use serde::{Serialize, Deserialize};
        
use super::{Prototype, type_stubs::*};
        
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ResearchAchievement {
    /// research_all :: bool (optional)
    research_all: Option<bool>,    

    /// technology :: string (optional)
    technology: Option<String>,    

}

impl Prototype for ResearchAchievement {
    const TYPE: Option<&'static str> = Some("research-achievement");
}


