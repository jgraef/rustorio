pub mod files;
pub mod lua;

use std::{
    cmp::Ordering,
    collections::{
        HashMap,
        HashSet,
    },
    fs::File,
    hash::{
        Hash,
        Hasher,
    },
    io::{
        BufReader,
        Read,
    },
    path::{
        Path,
        PathBuf,
    },
    str::FromStr,
    sync::Arc,
};

use byteorder::{
    LittleEndian,
    ReadBytesExt,
};
use lazy_static::lazy_static;
use mlua::{
    Table,
    Value,
};
use regex::Regex;
use rustorio_proptree::Value as PropertyTree;
use serde::Deserialize;
use thiserror::Error;

use self::{
    files::{
        ModFiles,
        PathError,
        Scopes,
    },
    lua::FactorioLua,
};
use crate::FromLuaValue;

#[derive(Debug, thiserror::Error)]
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

    #[error("zip error")]
    Zip(#[from] zip::result::ZipError),

    #[error("invalid path")]
    Path(#[from] PathError),

    #[error("file not found: {0}")]
    FileNotFound(PathBuf),
}

lazy_static! {
    static ref REGEX: Regex = Regex::new(r"(\d+)\.(\d+)(\.(\d+))?").unwrap();
}

#[derive(Debug, thiserror::Error)]
#[error("Could not parse version: {0}")]
pub struct VersionParseError(String);

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Version {
    pub major: u16,
    pub minor: u16,
    pub patch: u16,
}

impl Version {
    pub fn new(major: u16, minor: u16, patch: u16) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }
}

impl FromStr for Version {
    type Err = VersionParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let err = || VersionParseError(s.to_owned());

        if let Some(captures) = REGEX.captures(s) {
            let major = captures
                .get(1)
                .ok_or_else(err)?
                .as_str()
                .parse()
                .map_err(|_| err())?;
            let minor = captures
                .get(2)
                .ok_or_else(err)?
                .as_str()
                .parse()
                .map_err(|_| err())?;
            let patch = captures
                .get(4)
                .map(|m| m.as_str().parse())
                .transpose()
                .map_err(|_| err())?
                .unwrap_or_default();
            Ok(Version::new(major, minor, patch))
        }
        else {
            Err(err())
        }
    }
}

impl std::fmt::Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct InfoJson {
    pub name: String,

    pub version: String,

    pub title: String,

    pub author: String,

    pub contact: Option<String>,

    pub homepage: Option<String>,

    pub description: Option<String>,

    pub factorio_version: Option<String>,

    #[serde(default)]
    pub dependencies: Vec<String>,
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

        let i = if prefix == DependencyPrefix::Hard {
            0
        }
        else {
            1
        };

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

    pub fn check(&self, fmod: Option<&Mod>) -> Result<(), DependencyError> {
        match (self.prefix, fmod) {
            (DependencyPrefix::Hard, None) => {
                return Err(DependencyError::Missing(self.clone()));
            }
            (DependencyPrefix::Hard, Some(fmod)) => {
                if !self.version_matches(&fmod.version) {
                    return Err(DependencyError::Incompatible(
                        fmod.name().to_owned(),
                        self.clone(),
                    ));
                }
            }
            (DependencyPrefix::Incompatible, Some(fmod)) => {
                if self.version_matches(&fmod.version) {
                    return Err(DependencyError::Incompatible(
                        fmod.name().to_owned(),
                        self.clone(),
                    ));
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
    Cycle(Vec<(String, Dependency)>),

    #[error("Missing dependency: {0:?}")]
    Missing(Dependency),

    #[error("Dependency {1:?} incompatible with mod: {0:?}")]
    Incompatible(String, Dependency),

    #[error("Two conflicting dependencies: {0:?} and {1:?}")]
    Conflict(Dependency, Dependency),
}

struct DependencyChecker;

impl DependencyChecker {
    pub fn sort_mods(mods: &mut Mods) {
        mods.mods.sort_by(|a, b| {
            if a.depends_on(b).is_some() {
                Ordering::Greater
            }
            else if b.depends_on(a).is_some() {
                Ordering::Less
            }
            else {
                Ordering::Equal
            }
        });
    }

    pub fn check(mods: &Mods) -> Result<(), DependencyError> {
        let mut checked = HashSet::new();
        let mut chain = vec![];

        for fmod in mods.iter() {
            if !checked.contains(fmod.name()) {
                Self::check_mod(fmod.name(), mods, &mut chain, &mut checked)?;
            }
        }

        Ok(())
    }

    fn check_mod<'a>(
        mod_name: &'a str,
        mods: &'a Mods,
        chain: &mut Vec<(&'a str, &'a Dependency)>,
        checked: &mut HashSet<&'a str>,
    ) -> Result<(), DependencyError> {
        if let Some(i) = chain
            .iter()
            .position(|(other_mod, _)| mod_name == *other_mod)
        {
            let cycle = chain[i..]
                .iter()
                .map(|(mod_name, dep)| ((*mod_name).to_owned(), (*dep).clone()))
                .collect();
            return Err(DependencyError::Cycle(cycle));
        }

        for dep in mods.get(mod_name).as_ref().unwrap().dependencies.values() {
            let dep_mod = mods.get(&dep.mod_name).map(|m| m.as_ref());

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

#[derive(Debug)]
pub struct Mod {
    info: InfoJson,

    version: Version,

    factorio_version: Version,

    dependencies: HashMap<String, Dependency>,

    files: ModFiles,
}

impl Mod {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        log::info!("Loading mod: {}", path.as_ref().display());

        let path = path.as_ref();
        let files = ModFiles::open(path)?;
        let info: InfoJson = serde_json::from_slice(&files.read("info.json")?)?;

        let version = info.version.parse()?;
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
            info,
            version,
            factorio_version,
            dependencies,
            files,
        })
    }

    pub fn info(&self) -> &InfoJson {
        &self.info
    }

    pub fn factorio_version(&self) -> Version {
        self.factorio_version
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
    pub version: Version,
    pub dev_version: u16,
    pub settings: PropertyTree,
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

#[derive(Debug, Default)]
struct Mods {
    mods: Vec<Arc<Mod>>,
    by_name: HashMap<String, Arc<Mod>>,
}

impl Mods {
    pub fn insert(&mut self, fmod: Mod) {
        let fmod = Arc::new(fmod);
        let name = fmod.name().to_owned();
        self.mods.push(fmod.clone());
        self.by_name.insert(name, fmod);
    }

    pub fn iter<'a>(&'a self) -> impl Iterator<Item = &Arc<Mod>> + 'a {
        self.mods.iter()
    }

    pub fn get(&self, name: &str) -> Option<&Arc<Mod>> {
        self.by_name.get(name)
    }
}

pub struct Builder {
    core_path: PathBuf,
    mods: Mods,
    settings: Option<ModSettings>,
}

impl Builder {
    pub fn new(core: impl AsRef<Path>) -> Self {
        Self {
            core_path: core.as_ref().to_owned(),
            mods: Mods::default(),
            settings: None,
        }
    }

    pub fn from_data_dir(data_dir: impl AsRef<Path>) -> Result<Self, Error> {
        let data_dir = data_dir.as_ref();
        let mut builder = Self::new(data_dir.join("core"));
        builder.add_mod(data_dir.join("base"))?;
        Ok(builder)
    }

    pub fn add_mod(&mut self, path: impl AsRef<Path>) -> Result<(), Error> {
        let fmod = Mod::open(path)?;
        self.mods.insert(fmod);
        Ok(())
    }

    pub fn add_mod_dir<P: AsRef<Path>>(&mut self, mod_dir: P) -> Result<(), Error> {
        self.settings = Some(ModSettings::read_from_file(
            mod_dir.as_ref().join("mod-settings.dat"),
        )?);

        for r in mod_dir.as_ref().read_dir()? {
            let entry = r?;
            if entry.file_type()?.is_dir() && entry.path().join("info.json").exists()
                || entry.path().extension().and_then(|e| e.to_str()) == Some("zip")
            {
                self.add_mod(entry.path())?;
            }
        }

        Ok(())
    }

    pub fn finish(mut self) -> Result<Loader, Error> {
        DependencyChecker::check(&self.mods)?;
        DependencyChecker::sort_mods(&mut self.mods);

        let mods = Arc::new(self.mods);
        let scopes = Scopes::new(self.core_path, mods.clone());

        Ok(Loader {
            settings: self.settings,
            mods,
            scopes,
        })
    }
}

pub struct Loader {
    settings: Option<ModSettings>,
    mods: Arc<Mods>,
    scopes: Scopes,
}

impl Loader {
    pub fn builder(core_path: impl AsRef<Path>) -> Builder {
        Builder::new(core_path)
    }

    pub fn vanilla(data_dir: impl AsRef<Path>) -> Result<Self, Error> {
        Ok(Builder::from_data_dir(data_dir)?.finish()?)
    }

    pub fn modded(data_dir: impl AsRef<Path>, mod_dir: impl AsRef<Path>) -> Result<Self, Error> {
        let mut builder = Builder::from_data_dir(data_dir)?;
        builder.add_mod_dir(mod_dir)?;
        Ok(builder.finish()?)
    }

    fn run_with_all(&self, lua: &FactorioLua, file_name: impl AsRef<Path>) -> Result<(), Error> {
        let file_name = file_name.as_ref();

        for fmod in self.mods.iter() {
            let scope = self.scopes.mod_scope(fmod.clone());
            if scope.exists(file_name)? {
                lua.set_loader(scope.clone())?;
                lua.run_script_from_file(file_name, scope)?;
            }
        }

        Ok(())
    }

    fn settings_stage(&self) -> Result<(), Error> {
        let mut lua = FactorioLua::new()?;

        let mods = lua.create_table()?;
        // todo: populate table

        let globals = lua.globals();
        globals.set("mods", mods)?;
        drop(globals);

        self.run_with_all(&mut lua, "settings.lua")?;
        self.run_with_all(&mut lua, "settings-updates.lua")?;
        self.run_with_all(&mut lua, "settings-final-fixes.lua")?;

        // TODO: load mod-settings.dat

        Ok(())
    }

    pub fn data_stage<T: FromLuaValue>(&self) -> Result<T, crate::Error> {
        let mut lua = FactorioLua::new()?;

        // Initialize the lua context.
        let mods = lua.create_table()?; // TODO
        let settings = lua.create_table()?; // TODO

        let globals = lua.globals();
        globals.set("mods", mods)?;
        globals.set("settings", settings)?;
        drop(globals);

        let scope = self.scopes.core_scope();
        lua.set_loader(scope.clone())?;
        lua.run_script_from_file("lualib/dataloader.lua", scope.clone())?;
        lua.run_script_from_file("data.lua", scope)?;

        self.run_with_all(&mut lua, "data.lua")?;
        self.run_with_all(&mut lua, "data-updates.lua")?;
        self.run_with_all(&mut lua, "data-final-fixes.lua")?;

        let data_raw = lua
            .globals()
            .get::<_, Table>("data")?
            .get::<_, Value>("raw")?;

        Ok(T::from_lua_value(data_raw)?)
    }

    pub fn read_file(&self, path: impl AsRef<Path>) -> Result<Vec<u8>, Error> {
        self.scopes.unscoped().read(path)
    }
}
