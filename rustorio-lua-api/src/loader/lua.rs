use std::{
    fmt::Display,
    fs::File,
    io::{
        BufReader,
        Read,
    },
    ops::Deref,
    path::{
        Component,
        Path,
        PathBuf,
    },
    sync::Arc,
};

use mlua::{
    FromLuaMulti,
    Lua,
    StdLib,
    Table,
    Value,
};
use parking_lot::Mutex;
use zip::ZipArchive;

use super::{
    Error,
    Mod,
    Mods,
};
use crate::utils::HasNextExt;

pub fn lua_error(e: impl std::fmt::Display) -> mlua::Error {
    log::debug!("lua_error: {}", e);
    mlua::Error::RuntimeError(e.to_string())
}

pub struct FactorioLua {
    lua: Lua,
}

impl FactorioLua {
    pub fn new() -> Result<Self, Error> {
        // Don't use io, os or package. We provide our own `require` function.
        let libs = StdLib::TABLE | StdLib::STRING | StdLib::BIT | StdLib::MATH | StdLib::DEBUG;
        let lua = unsafe { Lua::unsafe_new_with(libs, Default::default()) };

        let defines = import_defines(&lua)?;
        lua.globals().set("defines", defines)?;

        Ok(Self { lua })
    }

    pub fn set_loader(&self, scope: Scope) -> Result<(), Error> {
        let require = self.lua.create_function(move |lua, name: String| {
            log::debug!("require called: {}", name);

            let module = scope.import(lua, &name).map_err(lua_error)?;
            Ok(module)
        })?;

        self.lua.globals().set("require", require)?;

        Ok(())
    }

    pub fn run_script<'lua, 's: 'lua, R: FromLuaMulti<'lua>>(
        &'lua self,
        name: impl Display,
        source: impl AsRef<[u8]>,
    ) -> Result<R, Error> {
        Ok(self
            .load(source.as_ref())
            .set_name(name.to_string())
            .eval()?)
    }

    pub fn run_script_from_file<'lua, 'p: 'lua, 'v, R: FromLuaMulti<'lua>>(
        &'lua self,
        path: impl AsRef<Path> + 'p,
        scope: Scope,
    ) -> Result<R, Error> {
        let path = path.as_ref();
        log::debug!("run script: {} ({:?})", path.display(), scope.scope_id());
        let code = scope.read(&path).map_err(lua_error)?;
        self.run_script(path.display(), code)
    }
}

impl Deref for FactorioLua {
    type Target = Lua;

    fn deref(&self) -> &Self::Target {
        &self.lua
    }
}

#[derive(Debug)]
pub enum ModFiles {
    Direcory {
        path: PathBuf,
    },
    Zip {
        path: PathBuf,
        zip: Mutex<ZipArchive<BufReader<File>>>,
    },
}

impl ModFiles {
    pub fn open(path: impl AsRef<Path>) -> Result<Self, Error> {
        let path = path.as_ref().to_owned();

        if path.is_dir() {
            Ok(Self::Direcory { path })
        }
        else {
            let zip = ZipArchive::new(BufReader::new(File::open(&path)?))?;
            Ok(Self::Zip {
                path,
                zip: Mutex::new(zip),
            })
        }
    }

    pub fn exists(&self, path: impl AsRef<Path>) -> bool {
        let file_path = path.as_ref();

        match self {
            ModFiles::Direcory { path } => {
                let path = path.join(file_path);
                path.exists()
            }
            ModFiles::Zip { zip, .. } => {
                let zip = zip.lock();
                let file_path = file_path.to_string_lossy();

                for file_name in zip.file_names() {
                    if file_name == file_path {
                        return true;
                    }
                }

                false
            }
        }
    }

    pub fn read(&self, path: impl AsRef<Path>) -> Result<Vec<u8>, Error> {
        let file_path = path.as_ref();

        match self {
            ModFiles::Direcory { path } => {
                let path = path.join(file_path);
                Ok(std::fs::read(&path)?)
            }
            ModFiles::Zip { zip, .. } => {
                let name = file_path.to_string_lossy();

                let mut zip = zip.lock();
                let mut file = zip.by_name(&name)?;

                let mut buf = Vec::with_capacity(file.size() as usize);
                file.read_to_end(&mut buf)?;

                Ok(buf)
            }
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
                import_paths: vec![".".into(), "__core__/lualib/".into()],
            }),
            mods,
        }
    }

    pub fn unscoped(&self) -> Scope {
        Scope {
            scopes: self.clone(),
            local: None,
        }
    }

    pub fn core_scope(&self) -> Scope {
        Scope {
            scopes: self.clone(),
            local: Some(Local::Core),
        }
    }

    pub fn mod_scope(&self, fmod: Arc<Mod>) -> Scope {
        Scope {
            scopes: self.clone(),
            local: Some(Local::Mod(fmod)),
        }
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
    local: Option<Local>,
}

impl Scope {
    fn get_local(&self, path: &ScopedPath) -> Option<Local> {
        match path.scope {
            Some(ScopeId::Core) => Some(Local::Core),
            Some(ScopeId::Mod(fmod)) => {
                self.scopes
                    .mods
                    .get(fmod)
                    .map(|fmod| Local::Mod(fmod.clone()))
            }
            None => self.local.clone(),
        }
    }

    pub fn exists(&self, path: impl AsRef<Path>) -> Result<bool, Error> {
        let path = path.as_ref();
        let path = ScopedPath::new(path)?;

        let Some(local) = self.get_local(&path)
        else {
            return Ok(false);
        };

        let exists = match local {
            Local::Core => {
                let path = self.scopes.data.core_path.join(path.path);
                path.exists()
            }
            Local::Mod(fmod) => fmod.files.exists(path.path),
        };

        Ok(exists)
    }

    pub fn read(&self, path: impl AsRef<Path>) -> Result<Vec<u8>, Error> {
        let path = path.as_ref();
        let scoped_path = ScopedPath::new(path)?;

        let Some(local) = self.get_local(&scoped_path)
        else {
            return Err(Error::FileNotFound(path.to_owned()));
        };

        let data = match local {
            Local::Core => {
                let path = self.scopes.data.core_path.join(scoped_path.path);
                std::fs::read(path)?
            }
            Local::Mod(fmod) => fmod.files.read(scoped_path.path)?,
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

    fn scope_id(&self) -> Option<ScopeId> {
        let scope_id = match self.local.as_ref()? {
            Local::Core => ScopeId::Core,
            Local::Mod(fmod) => ScopeId::Mod(fmod.name()),
        };
        Some(scope_id)
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
                Component::RootDir => {}
                Component::CurDir => {}
                _ => return Err(PathError::new(path)),
            }
        }

        let path = if scope.is_some() {
            components.as_path()
        }
        else {
            path
        };

        Ok(ScopedPath { scope, path })
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

pub fn get_data_raw(lua: &Lua) -> Result<mlua::Value, Error> {
    Ok(lua
        .globals()
        .get::<_, Table>("data")?
        .get::<_, Value>("raw")?)
}

pub fn import_defines(lua: &Lua) -> Result<mlua::Value, Error> {
    // run the following code in-game to export the defines
    // /c game.write_file('defines.lua', serpent.block(defines))

    const DEFINES: &'static str = include_str!("defines.lua");
    let defines = lua.load(DEFINES).set_name("defines").eval()?;

    Ok(defines)
}
