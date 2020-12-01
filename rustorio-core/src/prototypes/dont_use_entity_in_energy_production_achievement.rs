use serde::{Deserialize, Serialize};

use super::Prototype;
use crate::types::*; // TODO: Import only specific types

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DontUseEntityInEnergyProductionAchievement {
    /// excluded :: string or table of string
    excluded: Todo,

    /// included :: string or table of string
    included: Todo,

    /// last_hour_only :: bool (optional)
    last_hour_only: Option<bool>,

    /// minimum_energy_produced :: Energy (optional)
    minimum_energy_produced: Option<Energy>,
}

impl Prototype for DontUseEntityInEnergyProductionAchievement {
    const TYPE: Option<&'static str> = Some("dont-use-entity-in-energy-production-achievement");
}
