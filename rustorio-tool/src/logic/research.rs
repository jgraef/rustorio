use rustorio_prototype::{
    material::MaterialId,
    technology::TechnologyPrototype,
    Id,
};

use super::{
    materials::MaterialAmounts,
    time::{
        Duration,
        Instant,
    },
};

#[derive(Debug)]
pub enum Bottleneck {
    Labs,
    Production(MaterialId),
}

#[derive(Debug)]
pub struct ResearchEntry {
    pub technology: Id<TechnologyPrototype>,
    pub time_start: Instant,
    pub time_end: Instant,
    pub time: Duration,
    pub packs_needed: MaterialAmounts,
    pub packs_per_second: MaterialAmounts,
    pub bottleneck: Bottleneck,
}

pub fn calculate() {}
