use std::io::Write;

use color_eyre::eyre::Error;
use rustorio_core::{
    prototypes::{
        technology::{
            Modifier,
            TechnologyPrototype,
        },
        HasPrototypes,
        Id,
    },
    types::MaterialId,
};

use crate::{
    config::Config,
    materials::MaterialAmounts,
    time::{
        Clock,
        Duration,
        Instant,
    },
    Prototypes,
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

#[derive(Debug)]
pub struct Research {
    pub entries: Vec<ResearchEntry>,
}

#[derive(Debug)]
pub struct CheatSheet {
    pub research: Research,
}

impl CheatSheet {
    pub fn generate(config: &Config, prototypes: &Prototypes) -> Result<Self, Error> {
        let mut cheat_sheet = CheatSheet {
            research: Research { entries: vec![] },
        };

        let mut lab_setup_iter = config.research.lab_setup.iter().peekable();
        let mut lab_setup = lab_setup_iter.next().unwrap();
        let mut lab_speed_modifier: f64 = 1.0;
        let mut science_buffer = MaterialAmounts::default();
        let mut total_science_used = MaterialAmounts::default();
        let mut total_science_produced = MaterialAmounts::default();
        let mut clock = Clock::new(config.research.start_time);

        for tech_id in &config.research.order {
            let tech = HasPrototypes::try_get(prototypes, tech_id)?;
            let tech_data = tech.data.normal();

            if let Some(unit) = &tech_data.unit {
                let count = unit.count.as_count().unwrap() as f64;

                let packs_needed_total = MaterialAmounts::from_ingredients(&unit.ingredients)
                    * count
                    / lab_setup.productivity(prototypes);
                total_science_used += &packs_needed_total;

                let packs_from_buffer = science_buffer.elementwise_min(&packs_needed_total);
                science_buffer -= &packs_from_buffer;
                let packs_needed = &packs_needed_total - &packs_from_buffer;

                let (production_bottleneck, production_time) = (&packs_needed
                    / &config.research.science_production)
                    .argmax()
                    .unwrap();
                let production_time = Duration::from_seconds(production_time);

                let research_time = Duration::from_seconds(unit.time) * count
                    / lab_setup.speed(prototypes)
                    / lab_speed_modifier;

                let (bottleneck, time) = if production_time > research_time {
                    (
                        Bottleneck::Production(production_bottleneck),
                        production_time,
                    )
                }
                else {
                    (Bottleneck::Labs, research_time)
                };

                let packs_produced = &config.research.science_production * time.as_seconds();
                total_science_produced += &packs_produced;
                science_buffer += &packs_produced - &packs_needed;

                let time_start = clock.now();
                clock.advance(time);
                let time_end = clock.now();

                let packs_per_second = &packs_needed_total / research_time.as_seconds();

                cheat_sheet.research.entries.push(ResearchEntry {
                    technology: tech_id.clone(),
                    time_start,
                    time_end,
                    time,
                    packs_needed: packs_needed_total,
                    packs_per_second,
                    bottleneck,
                });
            }

            for effect in &tech_data.effects {
                match effect {
                    Modifier::LaboratorySpeed(modifier) => {
                        lab_speed_modifier += modifier.parent.modifier;
                    }
                    _ => {}
                }
            }

            if let Some(next_lab_setup) = lab_setup_iter.peek() {
                if next_lab_setup.after.as_ref().unwrap() == tech_id {
                    lab_setup = lab_setup_iter.next().unwrap();
                }
            }
        }

        Ok(cheat_sheet)
    }

    pub fn write<W: Write>(&self, _writer: W) -> Result<Self, std::io::Error> {
        todo!();
    }
}
