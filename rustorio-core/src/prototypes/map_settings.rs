use serde::{Serialize, Deserialize};
        
use super::{Prototype, type_stubs::*};
        
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MapSettings {
    /// difficulty_settings :: table
    difficulty_settings: Vec<Todo>,

    /// enemy_evolution :: table
    enemy_evolution: Vec<Todo>,

    /// enemy_expansion :: table
    enemy_expansion: Vec<Todo>,

    /// max_failed_behavior_count :: uint32
    max_failed_behavior_count: u32,    

    /// name :: string
    name: String,    

    /// path_finder :: table
    path_finder: Vec<Todo>,

    /// pollution :: table
    pollution: Vec<Todo>,

    /// steering :: table
    steering: Vec<Todo>,

    /// type :: string
    r#type: String,    

    /// unit_group :: table
    unit_group: Vec<Todo>,

}

impl Prototype for MapSettings {
    const TYPE: Option<&'static str> = Some("map-settings");
}


