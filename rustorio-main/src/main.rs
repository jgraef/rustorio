use std::collections::HashMap;

use rustorio_core::{
    mod_loader::ModLoader,
    error::Error,
    prototypes,
};
use rustorio_data_derive::FromLuaTable;


fn main() {
    dotenv::dotenv().ok();
    pretty_env_logger::init();

    if let Err(e) = run_app() {
        eprintln!("\n{}\n", e);
    }
}


pub type PrototypeMap<P> = HashMap<String, P>;


#[derive(Clone, Debug, FromLuaTable)]
struct Prototypes {
    accumulator: PrototypeMap<prototypes::accumulator::Accumulator>,
    achievement: PrototypeMap<prototypes::achievement::Achievement>,
    active_defense_equipment: PrototypeMap<prototypes::active_defense_equipment::ActiveDefenseEquipment>,
}


fn run_app() -> Result<(), Error> {
    let mut loader = ModLoader::new_with_base("data/core", "data/base")?;
    loader.check_dependencies()?;

    let prototypes: Prototypes = loader.data_stage()?;
    println!("{:#?}", prototypes);

    Ok(())
}
