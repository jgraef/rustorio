use std::path::PathBuf;

use serde::{
    Deserialize,
    Serialize,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    base_mod: PathBuf,
    mod_dir: PathBuf,
}
