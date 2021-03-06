use rustorio_core::blueprint::types::{SignalID, SignalType};

use lazy_static::lazy_static;


lazy_static! {
    pub static ref VANILLA_SIGNALS: Vec<SignalID> = vec![
        SignalID::new(SignalType::Fluid, "crude-oil".to_owned()),
        SignalID::new(SignalType::Fluid, "fluid-unknown".to_owned()),
        SignalID::new(SignalType::Fluid, "heavy-oil".to_owned()),
        SignalID::new(SignalType::Fluid, "light-oil".to_owned()),
        SignalID::new(SignalType::Fluid, "lubricant".to_owned()),
        SignalID::new(SignalType::Fluid, "petroleum-gas".to_owned()),
        SignalID::new(SignalType::Fluid, "steam".to_owned()),
        SignalID::new(SignalType::Fluid, "sulfuric-acid".to_owned()),
        SignalID::new(SignalType::Fluid, "water".to_owned()),
        SignalID::new(SignalType::Item, "accumulator".to_owned()),
        SignalID::new(SignalType::Item, "advanced-circuit".to_owned()),
        SignalID::new(SignalType::Item, "arithmetic-combinator".to_owned()),
        SignalID::new(SignalType::Item, "artillery-turret".to_owned()),
        SignalID::new(SignalType::Item, "assembling-machine-1".to_owned()),
        SignalID::new(SignalType::Item, "assembling-machine-2".to_owned()),
        SignalID::new(SignalType::Item, "assembling-machine-3".to_owned()),
        SignalID::new(SignalType::Item, "battery".to_owned()),
        SignalID::new(SignalType::Item, "battery-equipment".to_owned()),
        SignalID::new(SignalType::Item, "battery-mk2-equipment".to_owned()),
        SignalID::new(SignalType::Item, "beacon".to_owned()),
        SignalID::new(SignalType::Item, "belt-immunity-equipment".to_owned()),
        SignalID::new(SignalType::Item, "big-electric-pole".to_owned()),
        SignalID::new(SignalType::Item, "boiler".to_owned()),
        SignalID::new(SignalType::Item, "burner-generator".to_owned()),
        SignalID::new(SignalType::Item, "burner-inserter".to_owned()),
        SignalID::new(SignalType::Item, "burner-mining-drill".to_owned()),
        SignalID::new(SignalType::Item, "centrifuge".to_owned()),
        SignalID::new(SignalType::Item, "chemical-plant".to_owned()),
        SignalID::new(SignalType::Item, "coal".to_owned()),
        SignalID::new(SignalType::Item, "coin".to_owned()),
        SignalID::new(SignalType::Item, "computer".to_owned()),
        SignalID::new(SignalType::Item, "concrete".to_owned()),
        SignalID::new(SignalType::Item, "constant-combinator".to_owned()),
        SignalID::new(SignalType::Item, "construction-robot".to_owned()),
        SignalID::new(SignalType::Item, "copper-cable".to_owned()),
        SignalID::new(SignalType::Item, "copper-ore".to_owned()),
        SignalID::new(SignalType::Item, "copper-plate".to_owned()),
        SignalID::new(SignalType::Item, "crash-site-assembling-machine-1-broken".to_owned()),
        SignalID::new(SignalType::Item, "crash-site-assembling-machine-1-repaired".to_owned()),
        SignalID::new(SignalType::Item, "crash-site-assembling-machine-2-broken".to_owned()),
        SignalID::new(SignalType::Item, "crash-site-assembling-machine-2-repaired".to_owned()),
        SignalID::new(SignalType::Item, "crash-site-chest-1".to_owned()),
        SignalID::new(SignalType::Item, "crash-site-chest-2".to_owned()),
        SignalID::new(SignalType::Item, "crash-site-electric-pole".to_owned()),
        SignalID::new(SignalType::Item, "crash-site-generator".to_owned()),
        SignalID::new(SignalType::Item, "crash-site-lab-broken".to_owned()),
        SignalID::new(SignalType::Item, "crash-site-lab-repaired".to_owned()),
        SignalID::new(SignalType::Item, "crude-oil-barrel".to_owned()),
        SignalID::new(SignalType::Item, "decider-combinator".to_owned()),
        SignalID::new(SignalType::Item, "discharge-defense-equipment".to_owned()),
        SignalID::new(SignalType::Item, "electric-energy-interface".to_owned()),
        SignalID::new(SignalType::Item, "electric-engine-unit".to_owned()),
        SignalID::new(SignalType::Item, "electric-furnace".to_owned()),
        SignalID::new(SignalType::Item, "electric-mining-drill".to_owned()),
        SignalID::new(SignalType::Item, "electronic-circuit".to_owned()),
        SignalID::new(SignalType::Item, "empty-barrel".to_owned()),
        SignalID::new(SignalType::Item, "energy-shield-equipment".to_owned()),
        SignalID::new(SignalType::Item, "energy-shield-mk2-equipment".to_owned()),
        SignalID::new(SignalType::Item, "engine-unit".to_owned()),
        SignalID::new(SignalType::Item, "exoskeleton-equipment".to_owned()),
        SignalID::new(SignalType::Item, "explosives".to_owned()),
        SignalID::new(SignalType::Item, "express-loader".to_owned()),
        SignalID::new(SignalType::Item, "express-splitter".to_owned()),
        SignalID::new(SignalType::Item, "express-transport-belt".to_owned()),
        SignalID::new(SignalType::Item, "express-underground-belt".to_owned()),
        SignalID::new(SignalType::Item, "fast-inserter".to_owned()),
        SignalID::new(SignalType::Item, "fast-loader".to_owned()),
        SignalID::new(SignalType::Item, "fast-splitter".to_owned()),
        SignalID::new(SignalType::Item, "fast-transport-belt".to_owned()),
        SignalID::new(SignalType::Item, "fast-underground-belt".to_owned()),
        SignalID::new(SignalType::Item, "filter-inserter".to_owned()),
        SignalID::new(SignalType::Item, "flamethrower-turret".to_owned()),
        SignalID::new(SignalType::Item, "flying-robot-frame".to_owned()),
        SignalID::new(SignalType::Item, "fusion-reactor-equipment".to_owned()),
        SignalID::new(SignalType::Item, "gate".to_owned()),
        SignalID::new(SignalType::Item, "green-wire".to_owned()),
        SignalID::new(SignalType::Item, "gun-turret".to_owned()),
        SignalID::new(SignalType::Item, "hazard-concrete".to_owned()),
        SignalID::new(SignalType::Item, "heat-exchanger".to_owned()),
        SignalID::new(SignalType::Item, "heat-interface".to_owned()),
        SignalID::new(SignalType::Item, "heat-pipe".to_owned()),
        SignalID::new(SignalType::Item, "heavy-oil-barrel".to_owned()),
        SignalID::new(SignalType::Item, "infinity-chest".to_owned()),
        SignalID::new(SignalType::Item, "infinity-pipe".to_owned()),
        SignalID::new(SignalType::Item, "inserter".to_owned()),
        SignalID::new(SignalType::Item, "iron-chest".to_owned()),
        SignalID::new(SignalType::Item, "iron-gear-wheel".to_owned()),
        SignalID::new(SignalType::Item, "iron-ore".to_owned()),
        SignalID::new(SignalType::Item, "iron-plate".to_owned()),
        SignalID::new(SignalType::Item, "iron-stick".to_owned()),
        SignalID::new(SignalType::Item, "item-unknown".to_owned()),
        SignalID::new(SignalType::Item, "lab".to_owned()),
        SignalID::new(SignalType::Item, "land-mine".to_owned()),
        SignalID::new(SignalType::Item, "landfill".to_owned()),
        SignalID::new(SignalType::Item, "laser-turret".to_owned()),
        SignalID::new(SignalType::Item, "light-oil-barrel".to_owned()),
        SignalID::new(SignalType::Item, "loader".to_owned()),
        SignalID::new(SignalType::Item, "logistic-chest-active-provider".to_owned()),
        SignalID::new(SignalType::Item, "logistic-chest-buffer".to_owned()),
        SignalID::new(SignalType::Item, "logistic-chest-passive-provider".to_owned()),
        SignalID::new(SignalType::Item, "logistic-chest-requester".to_owned()),
        SignalID::new(SignalType::Item, "logistic-chest-storage".to_owned()),
        SignalID::new(SignalType::Item, "logistic-robot".to_owned()),
        SignalID::new(SignalType::Item, "long-handed-inserter".to_owned()),
        SignalID::new(SignalType::Item, "low-density-structure".to_owned()),
        SignalID::new(SignalType::Item, "lubricant-barrel".to_owned()),
        SignalID::new(SignalType::Item, "medium-electric-pole".to_owned()),
        SignalID::new(SignalType::Item, "night-vision-equipment".to_owned()),
        SignalID::new(SignalType::Item, "nuclear-fuel".to_owned()),
        SignalID::new(SignalType::Item, "nuclear-reactor".to_owned()),
        SignalID::new(SignalType::Item, "offshore-pump".to_owned()),
        SignalID::new(SignalType::Item, "oil-refinery".to_owned()),
        SignalID::new(SignalType::Item, "personal-laser-defense-equipment".to_owned()),
        SignalID::new(SignalType::Item, "personal-roboport-equipment".to_owned()),
        SignalID::new(SignalType::Item, "personal-roboport-mk2-equipment".to_owned()),
        SignalID::new(SignalType::Item, "petroleum-gas-barrel".to_owned()),
        SignalID::new(SignalType::Item, "pipe".to_owned()),
        SignalID::new(SignalType::Item, "pipe-to-ground".to_owned()),
        SignalID::new(SignalType::Item, "plastic-bar".to_owned()),
        SignalID::new(SignalType::Item, "player-port".to_owned()),
        SignalID::new(SignalType::Item, "power-switch".to_owned()),
        SignalID::new(SignalType::Item, "processing-unit".to_owned()),
        SignalID::new(SignalType::Item, "programmable-speaker".to_owned()),
        SignalID::new(SignalType::Item, "pump".to_owned()),
        SignalID::new(SignalType::Item, "pumpjack".to_owned()),
        SignalID::new(SignalType::Item, "radar".to_owned()),
        SignalID::new(SignalType::Item, "rail-chain-signal".to_owned()),
        SignalID::new(SignalType::Item, "rail-signal".to_owned()),
        SignalID::new(SignalType::Item, "red-wire".to_owned()),
        SignalID::new(SignalType::Item, "refined-concrete".to_owned()),
        SignalID::new(SignalType::Item, "refined-hazard-concrete".to_owned()),
        SignalID::new(SignalType::Item, "roboport".to_owned()),
        SignalID::new(SignalType::Item, "rocket-control-unit".to_owned()),
        SignalID::new(SignalType::Item, "rocket-fuel".to_owned()),
        SignalID::new(SignalType::Item, "rocket-part".to_owned()),
        SignalID::new(SignalType::Item, "rocket-silo".to_owned()),
        SignalID::new(SignalType::Item, "satellite".to_owned()),
        SignalID::new(SignalType::Item, "simple-entity-with-force".to_owned()),
        SignalID::new(SignalType::Item, "simple-entity-with-owner".to_owned()),
        SignalID::new(SignalType::Item, "small-electric-pole".to_owned()),
        SignalID::new(SignalType::Item, "small-lamp".to_owned()),
        SignalID::new(SignalType::Item, "small-plane".to_owned()),
        SignalID::new(SignalType::Item, "solar-panel".to_owned()),
        SignalID::new(SignalType::Item, "solar-panel-equipment".to_owned()),
        SignalID::new(SignalType::Item, "solid-fuel".to_owned()),
        SignalID::new(SignalType::Item, "splitter".to_owned()),
        SignalID::new(SignalType::Item, "stack-filter-inserter".to_owned()),
        SignalID::new(SignalType::Item, "stack-inserter".to_owned()),
        SignalID::new(SignalType::Item, "steam-engine".to_owned()),
        SignalID::new(SignalType::Item, "steam-turbine".to_owned()),
        SignalID::new(SignalType::Item, "steel-chest".to_owned()),
        SignalID::new(SignalType::Item, "steel-furnace".to_owned()),
        SignalID::new(SignalType::Item, "steel-plate".to_owned()),
        SignalID::new(SignalType::Item, "stone".to_owned()),
        SignalID::new(SignalType::Item, "stone-brick".to_owned()),
        SignalID::new(SignalType::Item, "stone-furnace".to_owned()),
        SignalID::new(SignalType::Item, "stone-wall".to_owned()),
        SignalID::new(SignalType::Item, "storage-tank".to_owned()),
        SignalID::new(SignalType::Item, "substation".to_owned()),
        SignalID::new(SignalType::Item, "sulfur".to_owned()),
        SignalID::new(SignalType::Item, "sulfuric-acid-barrel".to_owned()),
        SignalID::new(SignalType::Item, "train-stop".to_owned()),
        SignalID::new(SignalType::Item, "transport-belt".to_owned()),
        SignalID::new(SignalType::Item, "underground-belt".to_owned()),
        SignalID::new(SignalType::Item, "uranium-235".to_owned()),
        SignalID::new(SignalType::Item, "uranium-238".to_owned()),
        SignalID::new(SignalType::Item, "uranium-fuel-cell".to_owned()),
        SignalID::new(SignalType::Item, "uranium-ore".to_owned()),
        SignalID::new(SignalType::Item, "used-up-uranium-fuel-cell".to_owned()),
        SignalID::new(SignalType::Item, "water-barrel".to_owned()),
        SignalID::new(SignalType::Item, "wood".to_owned()),
        SignalID::new(SignalType::Item, "wooden-chest".to_owned()),
        SignalID::new(SignalType::Virtual, "signal-0".to_owned()),
        SignalID::new(SignalType::Virtual, "signal-1".to_owned()),
        SignalID::new(SignalType::Virtual, "signal-2".to_owned()),
        SignalID::new(SignalType::Virtual, "signal-3".to_owned()),
        SignalID::new(SignalType::Virtual, "signal-4".to_owned()),
        SignalID::new(SignalType::Virtual, "signal-5".to_owned()),
        SignalID::new(SignalType::Virtual, "signal-6".to_owned()),
        SignalID::new(SignalType::Virtual, "signal-7".to_owned()),
        SignalID::new(SignalType::Virtual, "signal-8".to_owned()),
        SignalID::new(SignalType::Virtual, "signal-9".to_owned()),
        SignalID::new(SignalType::Virtual, "signal-A".to_owned()),
        SignalID::new(SignalType::Virtual, "signal-B".to_owned()),
        SignalID::new(SignalType::Virtual, "signal-C".to_owned()),
        SignalID::new(SignalType::Virtual, "signal-D".to_owned()),
        SignalID::new(SignalType::Virtual, "signal-E".to_owned()),
        SignalID::new(SignalType::Virtual, "signal-F".to_owned()),
        SignalID::new(SignalType::Virtual, "signal-G".to_owned()),
        SignalID::new(SignalType::Virtual, "signal-H".to_owned()),
        SignalID::new(SignalType::Virtual, "signal-I".to_owned()),
        SignalID::new(SignalType::Virtual, "signal-J".to_owned()),
        SignalID::new(SignalType::Virtual, "signal-K".to_owned()),
        SignalID::new(SignalType::Virtual, "signal-L".to_owned()),
        SignalID::new(SignalType::Virtual, "signal-M".to_owned()),
        SignalID::new(SignalType::Virtual, "signal-N".to_owned()),
        SignalID::new(SignalType::Virtual, "signal-O".to_owned()),
        SignalID::new(SignalType::Virtual, "signal-P".to_owned()),
        SignalID::new(SignalType::Virtual, "signal-Q".to_owned()),
        SignalID::new(SignalType::Virtual, "signal-R".to_owned()),
        SignalID::new(SignalType::Virtual, "signal-S".to_owned()),
        SignalID::new(SignalType::Virtual, "signal-T".to_owned()),
        SignalID::new(SignalType::Virtual, "signal-U".to_owned()),
        SignalID::new(SignalType::Virtual, "signal-V".to_owned()),
        SignalID::new(SignalType::Virtual, "signal-W".to_owned()),
        SignalID::new(SignalType::Virtual, "signal-X".to_owned()),
        SignalID::new(SignalType::Virtual, "signal-Y".to_owned()),
        SignalID::new(SignalType::Virtual, "signal-Z".to_owned()),
        SignalID::new(SignalType::Virtual, "signal-anything".to_owned()),
        SignalID::new(SignalType::Virtual, "signal-black".to_owned()),
        SignalID::new(SignalType::Virtual, "signal-blue".to_owned()),
        SignalID::new(SignalType::Virtual, "signal-check".to_owned()),
        SignalID::new(SignalType::Virtual, "signal-cyan".to_owned()),
        SignalID::new(SignalType::Virtual, "signal-dot".to_owned()),
        SignalID::new(SignalType::Virtual, "signal-each".to_owned()),
        SignalID::new(SignalType::Virtual, "signal-everything".to_owned()),
        SignalID::new(SignalType::Virtual, "signal-green".to_owned()),
        SignalID::new(SignalType::Virtual, "signal-grey".to_owned()),
        SignalID::new(SignalType::Virtual, "signal-info".to_owned()),
        SignalID::new(SignalType::Virtual, "signal-pink".to_owned()),
        SignalID::new(SignalType::Virtual, "signal-red".to_owned()),
        SignalID::new(SignalType::Virtual, "signal-unknown".to_owned()),
        SignalID::new(SignalType::Virtual, "signal-white".to_owned()),
        SignalID::new(SignalType::Virtual, "signal-yellow".to_owned()),
    ];
}