use std::{io::{BufReader, Read}, fs::File, path::{PathBuf, Path, Component}, sync::Arc};

use mlua::{Value, Lua};
use parking_lot::Mutex;
use zip::ZipArchive;

use crate::utils::HasNextExt;

use super::{Error, Mods, Mod};


#[derive(Debug)]
pub enum ModFiles {
    Direcory {
        path: PathBuf,
    },
    Zip {
        path: PathBuf,
        zip: Mutex<ZipArchive<BufReader<File>>>,
    }
}

impl ModFiles {
    pub fn open(path: impl AsRef<Path>) -> Result<Self, Error> {
        let path = path.as_ref().to_owned();

        if path.is_dir() {
            Ok(Self::Direcory {
                path,
            })
        }
        else {
            let zip = ZipArchive::new(BufReader::new(File::open(&path)?))?;
            Ok(Self::Zip { path, zip: Mutex::new(zip) })
        }
    }

    pub fn exists(&self, path: impl AsRef<Path>) -> bool {
        let file_path = path.as_ref();

        match self {
            ModFiles::Direcory { path } => {
                let path = path.join(file_path);
                path.exists()
            },
            ModFiles::Zip { zip, .. } => {
                let zip = zip.lock();
                let file_path = file_path.to_string_lossy();

                for file_name in zip.file_names() {
                    if file_name == file_path {
                        return true;
                    }
                }

                false
            },
        }
    }

    pub fn read(&self, path: impl AsRef<Path>) -> Result<Vec<u8>, Error> {
        let file_path = path.as_ref();

        match self {
            ModFiles::Direcory { path } => {
                let path = path.join(file_path);
                Ok(std::fs::read(&path)?)
            },
            ModFiles::Zip { zip, .. } => {
                let name = file_path.to_string_lossy();

                let mut zip = zip.lock();
                let mut file = zip.by_name(&name)?;

                let mut buf = Vec::with_capacity(file.size() as usize);
                file.read_to_end(&mut buf)?;

                Ok(buf)
            },
        }
    }
}

#[derive(Debug)]
struct ScopesData {
    core_path: PathBuf,
    import_paths: Vec<PathBuf>,
}

#[derive(Debug, Clone)]
pub struct Scopes {
    data: Arc<ScopesData>,
    mods: Arc<Mods>,
}

impl Scopes {
    pub fn new(core_path: impl AsRef<Path>, mods: Arc<Mods>) -> Self {
        Self {
            data: Arc::new(ScopesData {
                core_path: core_path.as_ref().to_owned(),
                import_paths: vec![
                    ".".into(),
                    "__core__/lualib/".into(),
                ],
            }),
            mods
        }
    }

    pub fn core_scope(&self) -> Scope {
        Scope { scopes: self.clone(), local: Local::Core }
    }

    pub fn mod_scope(&self, fmod: Arc<Mod>) -> Scope {
        Scope { scopes: self.clone(), local: Local::Mod(fmod) }
    }
}

#[derive(Debug, Clone)]
enum Local {
    Core,
    Mod(Arc<Mod>),
}

#[derive(Debug, Clone)]
pub struct Scope {
    scopes: Scopes,
    local: Local,
}

impl Scope {
    fn get_local(&self, path: &ScopedPath) -> Option<Local> {
        match path.scope {
            Some(ScopeId::Core) => {
                Some(Local::Core)
            },
            Some(ScopeId::Mod(fmod)) => {
                self.scopes.mods.get(fmod)
                    .map(|fmod| Local::Mod(fmod.clone()))
            },
            None => Some(self.local.clone()),
        }
    }

    pub fn exists(&self, path: impl AsRef<Path>) -> Result<bool, Error> {
        let path = path.as_ref();
        let path = ScopedPath::new(path)?;

        let Some(local) = self.get_local(&path) else { return Ok(false) };

        let exists = match local {
            Local::Core => {
                let path = self.scopes.data.core_path.join(path.path);
                path.exists()
            },
            Local::Mod(fmod) => {
                fmod.files.exists(path.path)
            },
        };

        Ok(exists)
    }

    pub fn read(&self, path: impl AsRef<Path>) -> Result<Vec<u8>, Error> {
        let path = path.as_ref();
        let scoped_path = ScopedPath::new(path)?;

        let Some(local) = self.get_local(&scoped_path) else { return Err(Error::FileNotFound(path.to_owned())) };

        let data = match local {
            Local::Core => {
                let path = self.scopes.data.core_path.join(scoped_path.path);
                std::fs::read(path)?
            },
            Local::Mod(fmod) => {
                fmod.files.read(scoped_path.path)?
            },
        };

        Ok(data)
    }

    pub fn import<'lua>(&self, lua: &'lua Lua, name: &str) -> Result<Value<'lua>, Error> {
        let mut path = PathBuf::new();
        for (part, has_next) in name.split('.').has_next() {
            if has_next {
                path.push(part);
            }
            else {
                path.push(format!("{}.lua", part));
            }
        }

        for import_path in &self.scopes.data.import_paths {
            let path = import_path.join(&path);
            if self.exists(&path)? {
                let source = self.read(&path)?;
                let module: Value = lua.load(&source).set_name(path.to_string_lossy()).eval()?;
                return Ok(module);
            }
        }

        Err(Error::FileNotFound(path))
    }

    pub fn scope_id(&self) -> ScopeId {
        match &self.local {
            Local::Core => ScopeId::Core,
            Local::Mod(fmod) => ScopeId::Mod(fmod.name()),
        }
    }
}


#[derive(Debug)]
enum ScopeId<'a> {
    Core,
    Mod(&'a str),
}

#[derive(Debug)]
struct ScopedPath<'a> {
    scope: Option<ScopeId<'a>>,
    path: &'a Path,
}

impl<'a> ScopedPath<'a> {
    pub fn new(path: &'a Path) -> Result<Self, PathError> {
        let mut components = path.components();

        let mut scope = None;

        loop {
            let prefix = components.next().ok_or_else(|| PathError::new(path))?;
            match prefix {
                Component::Normal(s) => {
                    if let Some(s) = s.to_str() {
                        if let Some(s) = s.strip_prefix("__") {
                            if let Some(s) = s.strip_suffix("__") {
                                if s == "core" {
                                    scope = Some(ScopeId::Core);
                                }
                                else {
                                    scope = Some(ScopeId::Mod(s));
                                }
                            }
                        }
                    }
                    break;
                }
                Component::RootDir => {},
                Component::CurDir => {},
                _ => return Err(PathError::new(path)),
            }
        }

        let path = if scope.is_some() {
            components.as_path()
        }
        else {
            path
        };

        Ok(ScopedPath {
            scope,
            path,
        })
        
    }
}

#[derive(Debug, thiserror::Error)]
#[error("invalid path: {path}")]
pub struct PathError {
    path: PathBuf,
}

impl PathError {
    pub fn new(path: impl AsRef<Path>) -> Self {
        Self {
            path: path.as_ref().to_owned(),
        }
    }
}