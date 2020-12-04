use thiserror::Error;

use crate::{
    mod_loader::{DependencyError, DependencyParseError},
    version::VersionParseError,
};

#[derive(Debug, Error)]
pub enum Error {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Version parsing error: {0}")]
    Version(#[from] VersionParseError),

    #[error("Lua error: {0}")]
    Lua(#[from] mlua::Error),

    #[error("{0}")]
    DependencyParse(#[from] DependencyParseError),

    #[error("{0}")]
    Dependency(#[from] DependencyError),

    #[error("Duplicate mod: {0:?} and {1:?}")]
    DuplicateMod(String, String),

    #[error("Error while parsing property tree: {0}")]
    PropertyTree(#[from] rustorio_proptree::Error),

    // XXX
    #[error("{0}")]
    Format(#[from] std::fmt::Error),

    #[error("Error while converting Lua value: {0}")]
    LuaData(#[from] rustorio_data::Error),

    #[error("Missing prototype loader: {0}")]
    MissingPrototypeLoader(String),
}
