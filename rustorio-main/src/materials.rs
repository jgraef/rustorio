use std::{
    collections::HashMap,
    hash::Hash,
    ops::{
        Add,
        AddAssign,
        Div,
        DivAssign,
        Mul,
        MulAssign,
        Sub,
        SubAssign,
    },
};

use rustorio_prototype::{
    fluid::FluidPrototype,
    item::ItemPrototype,
    material::MaterialId,
    recipe::{
        IngredientPrototype,
        ProductPrototype,
    },
    types::{
        ItemOrFluid,
        MaterialType,
    },
    Id,
};
use serde::{
    Deserialize,
    Serialize,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct SparseVec<K: Clone + Eq + Hash, V> {
    values: HashMap<K, V>,
}

impl<K: Clone + Eq + Hash, V> SparseVec<K, V> {
    pub fn get(&self, key: &K) -> Option<&V> {
        self.values.get(key)
    }

    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        self.values.get_mut(key)
    }

    pub fn insert(&mut self, key: K, value: V) {
        self.values.insert(key, value);
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn map<F: Fn(&K, &V) -> U, U>(&self, f: F) -> SparseVec<K, U> {
        SparseVec {
            values: self
                .values
                .iter()
                .map(|(k, v)| {
                    let u = f(k, v);
                    (k.clone(), u)
                })
                .collect(),
        }
    }

    pub fn filter<F: Fn(&K, &V) -> bool>(self, f: F) -> SparseVec<K, V> {
        Self {
            values: self.values.into_iter().filter(|(k, v)| f(k, v)).collect(),
        }
    }

    pub fn zip<F: Fn(&K, Option<&V>, Option<&S>) -> Option<U>, S, U>(
        &self,
        other: &SparseVec<K, S>,
        f: F,
    ) -> SparseVec<K, U> {
        let mut values =
            HashMap::with_capacity(std::cmp::max(self.values.len(), other.values.len()));
        for (k, x) in &self.values {
            let y = other.values.get(&k);
            if let Some(z) = f(&k, Some(x), y) {
                values.insert(k.clone(), z);
            }
        }
        for (k, y) in &other.values {
            if !values.contains_key(&k) {
                if let Some(z) = f(&k, None, Some(y)) {
                    values.insert(k.clone(), z);
                }
            }
        }
        SparseVec { values }
    }

    pub fn iter(&self) -> impl Iterator<Item = (&K, &V)> {
        self.values.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&K, &mut V)> {
        self.values.iter_mut()
    }
}

impl<K: Clone + Eq + Hash, V> Default for SparseVec<K, V> {
    fn default() -> Self {
        Self {
            values: HashMap::new(),
        }
    }
}

#[derive(Clone, Default, Deserialize)]
#[serde(from = "SerializedMaterialAmounts")]
pub struct MaterialAmounts {
    pub items: SparseVec<Id<ItemPrototype>, f64>,
    pub fluids: SparseVec<Id<FluidPrototype>, f64>,
}

impl MaterialAmounts {
    pub fn from_ingredients(ingredients: &[IngredientPrototype]) -> Self {
        let mut materials = Self::default();

        for ingredient in ingredients {
            match &ingredient.item_or_fluid {
                ItemOrFluid::Item(item) => {
                    materials
                        .items
                        .insert(item.name.clone(), item.amount as f64);
                }
                ItemOrFluid::Fluid(fluid) => {
                    materials.fluids.insert(fluid.name.clone(), fluid.amount);
                }
            }
        }

        materials
    }

    pub fn from_products(products: &[ProductPrototype]) -> Self {
        let mut materials = Self::default();

        for product in products {
            match &product.item_or_fluid {
                ItemOrFluid::Item(item) => {
                    let amount = match (item.amount, item.amount_min, item.amount_max) {
                        (Some(amount), None, None) => amount as f64,
                        (None, Some(amount_min), Some(amount_max)) => {
                            0.5 * (amount_min as f64 + amount_max as f64)
                        }
                        _ => panic!("invalid ItemProductPrototype"),
                    };

                    materials
                        .items
                        .insert(item.name.clone(), amount * item.probability.unwrap_or(1.0));
                }
                ItemOrFluid::Fluid(fluid) => {
                    let amount = match (fluid.amount_min, fluid.amount_max) {
                        (None, None) => fluid.amount,
                        (Some(amount_min), Some(amount_max)) => 0.5 * (amount_min + amount_max),
                        _ => panic!("invalid FluidProductPrototype"),
                    };

                    materials.fluids.insert(
                        fluid.name.clone(),
                        amount * fluid.probability.unwrap_or(1.0),
                    );
                }
            }
        }

        materials
    }

    pub fn get_item(&self, id: &Id<ItemPrototype>) -> f64 {
        self.items.get(id).copied().unwrap_or_default()
    }

    pub fn get_fluid(&self, id: &Id<FluidPrototype>) -> f64 {
        self.fluids.get(id).copied().unwrap_or_default()
    }

    fn map<F: Fn(f64) -> f64>(&self, f: F) -> Self {
        Self {
            items: self.items.map(|_, x| f(*x)),
            fluids: self.fluids.map(|_, x| f(*x)),
        }
    }

    fn zip<F: Fn(f64, f64) -> f64>(&self, rhs: &Self, f: F) -> Self {
        let g = |lhs: Option<&f64>, rhs: Option<&f64>| {
            let lhs = lhs.copied().unwrap_or_default();
            let rhs = rhs.copied().unwrap_or_default();
            Some(f(lhs, rhs))
        };
        Self {
            items: self.items.zip(&rhs.items, |_, lhs, rhs| g(lhs, rhs)),
            fluids: self.fluids.zip(&rhs.fluids, |_, lhs, rhs| g(lhs, rhs)),
        }
    }

    pub fn iter<'a>(&'a self) -> impl Iterator<Item = (MaterialId, f64)> + 'a {
        self.items
            .iter()
            .map(|(k, v)| (MaterialId::Item(k.clone()), *v))
            .chain(
                self.fluids
                    .iter()
                    .map(|(k, v)| (MaterialId::Fluid(k.clone()), *v)),
            )
    }

    fn iter_values<'a>(&'a self) -> impl Iterator<Item = f64> + 'a {
        self.items
            .iter()
            .map(|(_, v)| v)
            .chain(self.fluids.iter().map(|(_, v)| v))
            .copied()
    }

    pub fn elementwise_min(&self, rhs: &Self) -> Self {
        self.zip(rhs, |lhs, rhs| lhs.min(rhs))
    }

    pub fn elementwise_max(&self, rhs: &Self) -> Self {
        self.zip(rhs, |lhs, rhs| lhs.max(rhs))
    }

    pub fn argmin(&self) -> Option<(MaterialId, f64)> {
        let mut it = self.iter();
        let mut min = it.next()?;
        while let Some(elem) = it.next() {
            if elem.1 < min.1 {
                min = elem;
            }
        }
        Some(min)
    }

    pub fn argmax(&self) -> Option<(MaterialId, f64)> {
        let mut it = self.iter();
        let mut max = it.next()?;
        while let Some(elem) = it.next() {
            if elem.1 > max.1 {
                max = elem;
            }
        }
        Some(max)
    }

    pub fn min(&self) -> Option<f64> {
        Some(self.argmin()?.1)
    }

    pub fn max(&self) -> Option<f64> {
        Some(self.argmax()?.1)
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct SerializedMaterialAmount {
    r#type: Option<MaterialType>,
    name: String,
    amount: f64,
}

#[derive(Debug, Serialize, Deserialize)]
struct SerializedMaterialAmounts(Vec<SerializedMaterialAmount>);

impl From<SerializedMaterialAmounts> for MaterialAmounts {
    fn from(value: SerializedMaterialAmounts) -> Self {
        let mut materials = MaterialAmounts::default();
        for amount in value.0 {
            match amount.r#type {
                Some(MaterialType::Item) | None => {
                    materials.items.insert(amount.name.into(), amount.amount);
                }
                Some(MaterialType::Fluid) => {
                    materials.fluids.insert(amount.name.into(), amount.amount);
                }
            }
        }
        materials
    }
}

impl Serialize for MaterialAmounts {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut amounts = Vec::with_capacity(self.items.len() + self.fluids.len());
        for (name, amount) in self.items.iter() {
            amounts.push(SerializedMaterialAmount {
                r#type: None,
                name: name.to_string(),
                amount: *amount,
            });
        }
        for (name, amount) in self.fluids.iter() {
            amounts.push(SerializedMaterialAmount {
                r#type: Some(MaterialType::Fluid),
                name: name.to_string(),
                amount: *amount,
            });
        }
        amounts.serialize(serializer)
    }
}

macro_rules! impl_elementwise {
    ($trait:ident, $method:ident, $op:tt) => {
        impl $trait<MaterialAmounts> for MaterialAmounts {
            type Output = MaterialAmounts;

            fn $method(self, rhs: MaterialAmounts) -> Self::Output {
                self.zip(&rhs, |lhs, rhs| lhs $op rhs)
            }
        }

        impl $trait<MaterialAmounts> for &MaterialAmounts {
            type Output = MaterialAmounts;

            fn $method(self, rhs: MaterialAmounts) -> Self::Output {
                self.zip(&rhs, |lhs, rhs| lhs $op rhs)
            }
        }

        impl $trait<&MaterialAmounts> for MaterialAmounts {
            type Output = MaterialAmounts;

            fn $method(self, rhs: &MaterialAmounts) -> Self::Output {
                self.zip(rhs, |lhs, rhs| lhs $op rhs)
            }
        }

        impl $trait<&MaterialAmounts> for &MaterialAmounts {
            type Output = MaterialAmounts;

            fn $method(self, rhs: &MaterialAmounts) -> Self::Output {
                self.zip(rhs, |lhs, rhs| lhs $op rhs)
            }
        }
    };
}

macro_rules! impl_elementwise_assign {
    ($trait:ident, $method:ident, $op:tt) => {
        impl $trait<MaterialAmounts> for MaterialAmounts {
            fn $method(&mut self, rhs: MaterialAmounts) {
                *self = &*self $op &rhs
            }
        }

        impl $trait<&MaterialAmounts> for MaterialAmounts {
            fn $method(&mut self, rhs: &MaterialAmounts) {
                *self = &*self $op rhs
            }
        }
    };
}

macro_rules! impl_scalar {
    ($trait:ident, $method:ident, $op:tt) => {
        impl $trait<f64> for MaterialAmounts {
            type Output = MaterialAmounts;

            fn $method(self, rhs: f64) -> Self::Output {
                self.map(|x| x $op rhs)
            }
        }

        impl $trait<f64> for &MaterialAmounts {
            type Output = MaterialAmounts;

            fn $method(self, rhs: f64) -> Self::Output {
                self.map(|x| x $op rhs)
            }
        }
    };
}

macro_rules! impl_scalar_assign {
    ($trait:ident, $method:ident, $op:tt) => {
        impl $trait<f64> for MaterialAmounts {
            fn $method(&mut self, rhs: f64) {
                *self = self.map(|x| x $op rhs)
            }
        }
    };
}

impl_elementwise!(Add, add, +);
impl_elementwise_assign!(AddAssign, add_assign, +);
impl_elementwise!(Sub, sub, -);
impl_elementwise_assign!(SubAssign, sub_assign, -);
impl_elementwise!(Mul, mul, *);
impl_elementwise_assign!(MulAssign, mul_assign, *);
impl_elementwise!(Div, div, /);
impl_elementwise_assign!(DivAssign, div_assign, /);
impl_scalar!(Mul, mul, *);
impl_scalar_assign!(MulAssign, mul_assign, *);
impl_scalar!(Div, div, /);
impl_scalar_assign!(DivAssign, div_assign, *);

impl std::fmt::Debug for MaterialAmounts {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut map = f.debug_map();
        for (id, amount) in self.items.iter() {
            map.entry(id, amount);
        }
        for (id, amount) in self.fluids.iter() {
            map.entry(id, amount);
        }
        map.finish()
    }
}
