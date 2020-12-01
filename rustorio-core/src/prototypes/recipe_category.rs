use serde::{Deserialize, Serialize};

use super::Prototype;
use crate::types::*; // TODO: Import only specific types

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RecipeCategory {}

impl Prototype for RecipeCategory {
    const TYPE: Option<&'static str> = Some("recipe-category");
}
