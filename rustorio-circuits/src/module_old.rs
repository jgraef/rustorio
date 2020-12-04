use std::collections::HashMap;

use crate::base::{Combinator, Wires};


pub struct ModuleComponent {
    pub module: String,

    pub port_wires: HashMap<String, Wire>,
}


pub enum Component {
    Combinator(Combinator),
    Module(ModuleComponent),
}


#[derive(Clone, Debug)]
pub struct Module {
    pub name: String,

    pub ports: HashMap<String, Wire>,

    pub wires: HashMap<String, usize>,

    pub components: HashMap<String, Component>,
}

impl Module {
    pub fn builder<N: AsRef<str>>(name: N) -> ModuleBuilder {
        ModuleBuilder::new(name)
    }
}


#[derive(Clone, Debug)]
pub struct ModuleBuilder {
    next_wire_id: usize,
    module: Module,
}

impl ModuleBuilder {
    pub fn new<N: AsRef<str>>(name: N) -> Self {
        Self {
            next_wire_id: 0,
            module: Module {
                name: name.as_ref().to_owned(),
                ports: HashMap::new(),
                components: HashMap::new(),
            }
        }
    }

    fn add_combinator<N: AsRef<str>>(mut self, name: N, combinator: Combinator) -> Self {
        self.module.components.insert(name.as_ref().to_owned(), combinator);
        self
    }
}
