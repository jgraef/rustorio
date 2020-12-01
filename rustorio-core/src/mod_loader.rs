use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    fs::File,
    hash::{Hash, Hasher},
    io::{BufReader, Read},
    path::{Path, PathBuf},
    str::FromStr,
    sync::Arc,
};

use byteorder::{LittleEndian, ReadBytesExt};
use mlua::Lua;
use serde::Deserialize;
use thiserror::Error;

use rustorio_proptree::Value as PropertyTree;
use rustorio_data::value::Value;

use crate::{error::Error, lua_utils, version::Version};
use std::convert::TryInto;

#[derive(Clone, Debug, Deserialize)]
pub struct InfoJson {
    name: String,

    version: String,

    title: String,

    author: String,

    contact: Option<String>,

    homepage: Option<String>,

    description: Option<String>,

    factorio_version: Option<String>,

    #[serde(default)]
    dependencies: Vec<String>,
}

#[derive(Debug, Error)]
#[error("Failed to parse dependency: {0}")]
pub struct DependencyParseError(String);

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DependencyPrefix {
    Hard,
    Incompatible,
    Optional,
    HiddenOptional,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DependencyOperator {
    Less,
    LessEqual,
    Equal,
    GreaterEqual,
    Greater,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Dependency {
    prefix: DependencyPrefix,
    mod_name: String,
    operator: DependencyOperator,
    version: Version,
}

impl FromStr for Dependency {
    type Err = DependencyParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let err = || DependencyParseError(s.to_owned());

        let parts = s.split_whitespace().collect::<Vec<&str>>();

        if parts.len() < 3 || parts.len() > 4 {
            return Err(err());
        }

        let prefix = match parts[0] {
            "!" => DependencyPrefix::Incompatible,
            "?" => DependencyPrefix::Optional,
            "(?)" => DependencyPrefix::HiddenOptional,
            _ => DependencyPrefix::Hard,
        };

        let i = if prefix == DependencyPrefix::Hard { 0 } else { 1 };

        let mod_name = parts[i].to_owned();

        let operator = match parts[i + 1] {
            "<" => DependencyOperator::Less,
            "<=" => DependencyOperator::LessEqual,
            "=" => DependencyOperator::Equal,
            ">=" => DependencyOperator::GreaterEqual,
            ">" => DependencyOperator::Greater,
            _ => return Err(err()),
        };

        let version = parts[i + 2].parse().map_err(|_| err())?;

        Ok(Self {
            prefix,
            mod_name,
            operator,
            version,
        })
    }
}

impl Dependency {
    fn version_matches(&self, version: &Version) -> bool {
        log::debug!("version_matches: {:?}, version={}", self, version);

        match self.operator {
            DependencyOperator::Less => *version < self.version,
            DependencyOperator::LessEqual => *version <= self.version,
            DependencyOperator::Equal => *version == self.version,
            DependencyOperator::GreaterEqual => *version >= self.version,
            DependencyOperator::Greater => *version > self.version,
        }
    }

    pub fn check(&self, fmod: Option<&Arc<Mod>>) -> Result<(), DependencyError> {
        match (self.prefix, fmod) {
            (DependencyPrefix::Hard, None) => {
                return Err(DependencyError::Missing(self.clone()));
            }
            (DependencyPrefix::Hard, Some(fmod)) => {
                if !self.version_matches(&fmod.version) {
                    return Err(DependencyError::Incompatible(Arc::clone(fmod), self.clone()));
                }
            }
            (DependencyPrefix::Incompatible, Some(fmod)) => {
                if self.version_matches(&fmod.version) {
                    return Err(DependencyError::Incompatible(Arc::clone(fmod), self.clone()));
                }
            }
            _ => {}
        }

        Ok(())
    }
}

#[derive(Debug, Error)]
pub enum DependencyError {
    #[error("Dependency cycle detected")]
    Cycle(Vec<(Arc<Mod>, Dependency)>),

    #[error("Missing dependency: {0:?}")]
    Missing(Dependency),

    #[error("Dependency {1:?} incompatible with mod: {0:?}")]
    Incompatible(Arc<Mod>, Dependency),

    #[error("Two conflicting dependencies: {0:?} and {1:?}")]
    Conflict(Dependency, Dependency),
}

struct DependencyChecker;

impl DependencyChecker {
    pub fn sort_mods(mods: &HashMap<String, Arc<Mod>>) -> Vec<Arc<Mod>> {
        let mut mods = mods.values().map(|fmod| Arc::clone(fmod)).collect::<Vec<Arc<Mod>>>();

        mods.sort_by(|a, b| {
            if a.depends_on(b).is_some() {
                Ordering::Greater
            } else if b.depends_on(a).is_some() {
                Ordering::Less
            } else {
                Ordering::Equal
            }
        });

        mods
    }

    pub fn check(mods: &HashMap<String, Arc<Mod>>) -> Result<(), DependencyError> {
        let mut checked = HashSet::new();
        let mut chain = vec![];

        for fmod in mods.values() {
            if !checked.contains(fmod.name()) {
                Self::check_mod(fmod.name(), mods, &mut chain, &mut checked)?;
            }
        }

        Ok(())
    }

    fn check_mod<'a>(
        mod_name: &'a str,
        mods: &'a HashMap<String, Arc<Mod>>,
        chain: &mut Vec<(&'a str, &'a Dependency)>,
        checked: &mut HashSet<&'a str>,
    ) -> Result<(), DependencyError> {
        if let Some(i) = chain.iter().position(|(other_mod, _)| mod_name == *other_mod) {
            let cycle = chain[i..]
                .iter()
                .map(|(mod_name, dep)| (Arc::clone(mods.get(*mod_name).as_ref().unwrap()), (*dep).clone()))
                .collect();
            return Err(DependencyError::Cycle(cycle));
        }

        for dep in mods.get(mod_name).as_ref().unwrap().dependencies.values() {
            let dep_mod = mods.get(&dep.mod_name);

            dep.check(dep_mod)?;

            if let Some(dep_mod) = dep_mod {
                chain.push((mod_name, dep));

                Self::check_mod(dep_mod.name(), mods, chain, checked)?;

                {
                    let (other_mod_name, other_dep) = chain.pop().unwrap();
                    assert_eq!((mod_name, dep), (other_mod_name, other_dep));
                }
            }
        }

        checked.insert(mod_name);

        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct Mod {
    path: PathBuf,

    info: InfoJson,

    version: Version,

    factorio_version: Version,

    dependencies: HashMap<String, Dependency>,
}

impl Mod {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        log::info!("Loading mod: {}", path.as_ref().display());

        let info_path = path.as_ref().join("info.json");
        let reader = BufReader::new(File::open(info_path)?);

        let info: InfoJson = serde_json::from_reader(reader)?;

        let version = info.version.parse()?;

        // TODO: check if path matches mod name and version

        let factorio_version = info
            .factorio_version
            .as_ref()
            .map(|s| s.parse())
            .transpose()?
            .unwrap_or_else(|| Version::new(0, 12, 0));

        let mut dependencies = HashMap::new();
        for s in &info.dependencies {
            let dep: Dependency = s.parse()?;
            if let Some(dep2) = dependencies.insert(dep.mod_name.clone(), dep.clone()) {
                log::error!("Conflicting dependencies in mod {}:", info.name);
                log::error!(" 1. dependency: {:?}", dep2);
                log::error!(" 2. dependency: {:?}", dep);
                return Err(DependencyError::Conflict(dep2.clone(), dep.clone()).into());
            }
        }

        Ok(Mod {
            path: path.as_ref().to_owned(),
            info,
            version,
            factorio_version,
            dependencies,
        })
    }

    pub fn run_lua<P: AsRef<Path>>(&self, lua: &mut Lua, file_name: P) -> Result<(), Error> {
        let source_path = self.path.join(file_name.as_ref());

        if !source_path.exists() {
            log::info!("Skipping {}", source_path.display());
            return Ok(());
        }

        log::debug!("Running {} on {}: {}", file_name.as_ref().display(), self.name(), source_path.display());

        lua_utils::set_mod_path(lua, &self.path)?;
        lua_utils::run_file(lua, source_path)?;

        Ok(())
    }

    pub fn name(&self) -> &str {
        &self.info.name
    }

    pub fn depends_on(&self, other: &Self) -> Option<&Dependency> {
        self.dependencies.get(other.name())
    }
}

impl Eq for Mod {}

impl PartialEq for Mod {
    fn eq(&self, other: &Self) -> bool {
        self.info.name == other.info.name
    }
}

impl Hash for Mod {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.info.name.hash(state);
    }
}

#[derive(Clone, Debug)]
pub struct ModSettings {
    version: Version,
    dev_version: u16,
    settings: PropertyTree,
}

impl ModSettings {
    pub fn read<R: Read>(mut reader: R) -> Result<Self, Error> {
        let version = Version::new(
            reader.read_u16::<LittleEndian>()?.into(),
            reader.read_u16::<LittleEndian>()?.into(),
            reader.read_u16::<LittleEndian>()?.into(),
        );

        let dev_version = reader.read_u16::<LittleEndian>()?.into();

        let _ = reader.read_u8()?;

        let mut buf = vec![];
        reader.read_to_end(&mut buf)?;

        let settings = rustorio_proptree::from_slice(&buf)?;

        Ok(ModSettings {
            version,
            dev_version,
            settings,
        })
    }

    pub fn read_from_file<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        Self::read(BufReader::new(File::open(path)?))
    }
}

pub struct ModLoader {
    core_path: PathBuf,
    settings: Option<ModSettings>,
    mods: HashMap<String, Arc<Mod>>,
    mods_sorted: Vec<Arc<Mod>>,
}

impl ModLoader {
    pub fn new<P>(core_path: P) -> Result<Self, Error>
    where
        P: AsRef<Path>,
    {
        Ok(Self {
            core_path: core_path.as_ref().to_owned(),
            settings: None,
            mods: HashMap::new(),
            mods_sorted: vec![],
        })
    }

    pub fn new_with_base<P, Q>(core_path: P, base_path: Q) -> Result<Self, Error>
    where
        P: AsRef<Path>,
        Q: AsRef<Path>,
    {
        let mut loader = Self::new(core_path)?;
        loader.add_base_mod(base_path)?;
        Ok(loader)
    }

    pub fn new_full<P, Q, R>(core_path: P, base_path: Q, mod_dir: R) -> Result<Self, Error>
    where
        P: AsRef<Path>,
        Q: AsRef<Path>,
        R: AsRef<Path>,
    {
        let mut loader = Self::new_with_base(core_path, base_path)?;
        loader.add_mod_dir(mod_dir)?;
        Ok(loader)
    }

    pub fn add_base_mod<P: AsRef<Path>>(&mut self, path: P) -> Result<Arc<Mod>, Error> {
        let base = Arc::new(Mod::open(path)?);
        let r = self.mods.insert("base".to_owned(), Arc::clone(&base));
        assert!(r.is_none());
        Ok(base)
    }

    pub fn add_mod_dir<P: AsRef<Path>>(&mut self, mod_dir: P) -> Result<(), Error> {
        // TODO: Store mod_settings path so we can store settings later.
        self.settings = Some(ModSettings::read_from_file(mod_dir.as_ref().join("mod-settings.dat"))?);

        log::debug!("Scanning mod dir: {}", mod_dir.as_ref().display());

        for r in mod_dir.as_ref().read_dir()? {
            let entry = r?;
            if entry.file_type()?.is_dir() && entry.path().join("info.json").exists() {
                let fmod = Arc::new(Mod::open(entry.path())?);

                if let Some(other_mod) = self.mods.insert(fmod.name().to_owned(), Arc::clone(&fmod)) {
                    return Err(Error::DuplicateMod(other_mod.name().to_owned(), fmod.name().to_owned()));
                }
            }
        }

        Ok(())
    }

    pub fn check_dependencies(&mut self) -> Result<(), Error> {
        DependencyChecker::check(&self.mods)?;
        self.mods_sorted = DependencyChecker::sort_mods(&self.mods);
        Ok(())
    }

    pub fn mods<'a>(&'a self) -> impl Iterator<Item = Arc<Mod>> + 'a {
        self.mods_sorted.iter().map(|fmod| Arc::clone(fmod))
    }

    pub fn run_with_all<P: AsRef<Path>>(&self, lua: &mut Lua, file_name: P) -> Result<(), Error> {
        for fmod in &self.mods_sorted {
            fmod.run_lua(lua, &file_name)?;
        }
        Ok(())
    }

    pub fn settings_stage(&mut self) -> Result<(), Error> {
        let mut lua = Lua::new();

        let mods = lua.create_table()?; // TODO

        let globals = lua.globals();
        globals.set("mods", mods)?;
        drop(globals);

        self.run_with_all(&mut lua, "settings.lua")?;
        self.run_with_all(&mut lua, "settings-updates.lua")?;
        self.run_with_all(&mut lua, "settings-final-fixes.lua")?;

        // TODO: load mod-settings.dat

        Ok(())
    }

    pub fn data_stage(&mut self) -> Result<Value, Error> {
        let mut lua = unsafe { Lua::unsafe_new() };

        if self.mods_sorted.is_empty() {
            log::warn!("mods_sorted is empty. Did you forget to call check_dependencies()?")
        }

        // Initialize the lua context.
        let mods = lua.create_table()?; // TODO
        let settings = lua.create_table()?; // TODO

        let globals = lua.globals();
        globals.set("mods", mods)?;
        globals.set("settings", settings)?;
        drop(globals);

        lua_utils::set_mod_path(&mut lua, "data/core")?;

        lua_utils::run_file(&mut lua, "data/data-preload.lua")?;
        lua_utils::run_file(&mut lua, "data/core/lualib/dataloader.lua")?;

        lua_utils::run_file(&mut lua, "data/core/data.lua")?;

        self.run_with_all(&mut lua, "data.lua")?;
        self.run_with_all(&mut lua, "data-updates.lua")?;
        self.run_with_all(&mut lua, "data-final-fixes.lua")?;

        let data_raw = lua_utils::get_data_raw(&lua)?;
        Ok(data_raw.try_into()?)
    }
}

#[cfg(test)]
mod tests {
    use mlua::Lua;

    use super::*;

    #[test]
    fn math_pow_exists() {
        let mut lua = Lua::new();

        lua_utils::run_code(
            &mut lua,
            r#"
            x = math.pow(2, 4)
        "#,
            "test",
        )
        .unwrap();

        let x: u32 = lua.globals().get("x").unwrap();
        assert_eq!(x, 16);
    }

    #[test]
    fn it_loads_base_mod() {
        let base_mod = Mod::open("data/base").unwrap();
        assert_eq!(base_mod.name(), "base");
    }

    #[test]
    fn it_loads_mods() {
        pretty_env_logger::init();

        let mut mod_loader = ModLoader::new_full("data/core", "data/base", "data/mods").unwrap();
        let mods = mod_loader.mods().collect::<Vec<Arc<Mod>>>();

        log::info!("Mods found");
        for fmod in mods {
            log::info!(" - {}", fmod.name());
        }

        //log::info!("Settings stage...");
        //mod_loader.settings_stage().unwrap();

        log::info!("Data stage...");
        mod_loader.data_stage().unwrap();
    }

    #[test]
    fn test_data_stage() {}

    #[test]
    fn it_loads_mod_settings() {
        let mod_settings = ModSettings::read_from_file("data/mods/mod-settings.dat").unwrap();

        log::debug!("Mod settings: {:#?}", mod_settings);
    }
}
