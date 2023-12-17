#[cfg(feature = "serde")]
use serde::{
    Deserialize,
    Serialize,
};

use super::{
    fluid::FluidPrototype,
    item::ItemPrototype,
    Id,
};
use crate::types::MaterialType;

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum MaterialId {
    Item(Id<ItemPrototype>),
    Fluid(Id<FluidPrototype>),
}

impl MaterialId {
    pub fn ty(&self) -> MaterialType {
        match self {
            MaterialId::Item(_) => MaterialType::Item,
            MaterialId::Fluid(_) => MaterialType::Fluid,
        }
    }

    pub fn as_item(&self) -> Option<&Id<ItemPrototype>> {
        match self {
            MaterialId::Item(item) => Some(item),
            MaterialId::Fluid(_) => None,
        }
    }

    pub fn as_fluid(&self) -> Option<&Id<FluidPrototype>> {
        match self {
            MaterialId::Item(_) => None,
            MaterialId::Fluid(fluid) => Some(fluid),
        }
    }
}
