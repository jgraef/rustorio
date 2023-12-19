use rustorio_prototype::{
    item::ModulePrototype,
    HasPrototypes,
    Id,
    Prototypes,
};
use serde::{
    Deserialize,
    Serialize,
};

#[derive(Default, Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Modules(pub Vec<Id<ModulePrototype>>);

impl Modules {
    pub fn speed(&self, prototypes: &Prototypes) -> f64 {
        let mut speed = 1.;
        for module in &self.0 {
            let Some(module) = HasPrototypes::<ModulePrototype>::get(prototypes, module)
            else {
                continue;
            };
            speed += module
                .effect
                .speed
                .as_ref()
                .map(|v| v.bonus)
                .unwrap_or_default();
        }
        speed
    }

    pub fn productivity(&self, prototypes: &Prototypes) -> f64 {
        let mut productivity = 1.;
        for module in &self.0 {
            let Some(module) = HasPrototypes::<ModulePrototype>::get(prototypes, module)
            else {
                continue;
            };
            productivity += module
                .effect
                .productivity
                .as_ref()
                .map(|v| v.bonus)
                .unwrap_or_default();
        }
        productivity
    }
}
