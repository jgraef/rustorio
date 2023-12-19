mod parser;

use std::{
    collections::HashMap,
    path::Path,
    string::FromUtf8Error,
};

#[cfg(feature = "serde")]
use serde::{
    Deserialize,
    Serialize,
};

use crate::{
    loader::files::ModFiles,
    Error,
};

#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("{0}")]
    Nom(String),
    #[error("utf-8 error")]
    Utf8(#[from] FromUtf8Error),
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize), serde(transparent))]
pub enum LocalizationElement {
    Text(String),
    Parameter(u32),
    Entity(String),
    Item(String),
    Tile(String),
    Fluid(String),
    Pluralization {
        parameter: u32,
        rules: Vec<PluralizationRule>,
    },
    // todo
    Todo,
}

#[derive(Clone, Debug)]
pub struct PluralizationRule {
    pub pattern: PluralizationPattern,
    pub text: String,
}

#[derive(Clone, Debug)]
pub enum PluralizationPattern {
    Numbers(Vec<u32>),
    EndsWith(Vec<u32>),
    Rest,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize), serde(transparent))]
pub struct Localization {
    pub elements: Vec<LocalizationElement>,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize), serde(transparent))]
pub struct Category {
    pub values: HashMap<String, Localization>,
}

impl Category {
    pub fn merge_into(&mut self, other: Category) {
        self.values.extend(other.values);
    }
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Locale {
    pub global: Category,
    pub categories: HashMap<String, Category>,
}

impl Locale {
    pub fn load(files: &ModFiles, language: &str) -> Result<Self, Error> {
        let mut locale = Self::default();
        let dir = Path::new("locale").join(language);

        for file_name in files.list_dir(&dir)? {
            let data = files.read(&file_name)?;
            let data = String::from_utf8(data).map_err(ParseError::from)?;
            let data = Self::parse(&data)?;
            locale.merge_into(data);
        }

        Ok(locale)
    }

    pub fn parse(input: &str) -> Result<Locale, ParseError> {
        match parser::parse_locale(input) {
            Ok((_, locale)) => Ok(locale),
            Err(nom::Err::Error(e)) => Err(ParseError::Nom(nom::error::convert_error(input, e))),
            _ => panic!("unexpected"),
        }
    }

    pub fn merge_into(&mut self, other: Locale) {
        self.global.merge_into(other.global);
        for (name, category) in other.categories {
            let into_category = self.categories.entry(name).or_insert_with(Default::default);
            into_category.merge_into(category);
        }
    }
}
