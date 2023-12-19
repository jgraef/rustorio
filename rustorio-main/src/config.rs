use rustorio_prototype::{
    item::ModulePrototype,
    technology::TechnologyPrototype,
    HasPrototypes,
    Id,
    Prototypes,
};
use serde::{
    Deserialize,
    Serialize,
};

use crate::{
    materials::MaterialAmounts,
    time::Instant,
};


#[derive(Debug, Deserialize)]
pub struct Config {
    pub research: ResearchConfig,
}

#[derive(Debug, Deserialize)]
pub struct ResearchConfig {
    pub start_time: Instant,
    pub order: Vec<Id<TechnologyPrototype>>,
    pub lab_setup: Vec<LabSetup>,
    pub science_production: MaterialAmounts,
}

#[derive(Debug, Deserialize)]
pub struct LabSetup {
    pub after: Option<Id<TechnologyPrototype>>,
    pub count: u32,
    #[serde(default)]
    pub modules: Modules,
}

impl LabSetup {
    pub fn speed(&self, prototypes: &Prototypes) -> f64 {
        // todo: modules
        self.count as f64 * self.modules.speed(prototypes)
    }

    pub fn productivity(&self, prototypes: &Prototypes) -> f64 {
        self.modules.productivity(prototypes)
    }
}

impl Default for LabSetup {
    fn default() -> Self {
        Self {
            after: None,
            count: 1,
            modules: Modules::default(),
        }
    }
}
