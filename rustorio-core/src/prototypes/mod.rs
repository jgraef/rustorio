mod visitor;
mod prototypes;

use std::{
    sync::Arc,
    collections::HashMap,
};

use legion::Resources;
use mlua::Table;

pub use visitor::Visitor;
pub use prototypes::*;

use crate::error::Error;


pub trait PrototypeInheritance {
    fn accept<V: Visitor>(&self, _visitor: &mut V) -> Result<(), V::Error>;

    fn load_into(resources: &mut Resources, name: String, table: Table) -> Result<(), Error>;
}

impl<P: Prototype> PrototypeInheritance for P {
    fn accept<V: Visitor>(&self, visitor: &mut V) -> Result<(), V::Error> {
        P::accept(self, visitor)
    }

    fn load_into(resources: &mut Resources, name: String, table: Table) -> Result<(), Error> {
        P::load_into(resources, name, table)
    }
}

impl PrototypeInheritance for () {
    fn accept<V: Visitor>(&self, _visitor: &mut V) -> Result<(), V::Error> { Ok(()) }
    fn load_into(_resources: &mut Resources, _name: String, _table: Table) -> Result<(), Error> { Ok(()) }
}

pub struct NameVisitor {
    name: Option<String>,
}


pub trait Prototype {
    const TYPE: Option<&'static str> = None;

    type Inherits: PrototypeInheritance = ();

    fn name(&self) -> &str {
        // TODO: "Cast" to PrototypeBase and get name
        todo!()
    }

    fn accept<V: Visitor>(&self, _visitor: &mut V) -> Result<(), V::Error> { todo!() }

    fn load_into(_resources: &mut Resources, _name: String, _table: Table) -> Result<(), Error> {
        /*let proto = Self::from_lua_table(table.clone())?;
        let mut proto_res = resources.get_mut_or_default::<PrototypeResource<Self>>();
        proto_res.prototypes.insert(name, proto);
        Ok(())*/
        todo!() // TODO: All prototypes must actually implement FromLuaTable
    }
}


#[derive(Clone, Debug, Default)]
pub struct PrototypeResource<P: Prototype> {
    prototypes: HashMap<String, Arc<P>>,
}


