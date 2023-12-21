use std::{
    fmt::Display,
    ops::Deref,
    path::Path,
};

use mlua::{
    FromLuaMulti,
    Lua,
    StdLib,
};

use super::{
    files::Scope,
    Error,
};

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

pub fn import_defines(lua: &Lua) -> Result<mlua::Value, Error> {
    // run the following code in-game to export the defines
    // /c game.write_file('defines.lua', serpent.block(defines))

    const DEFINES: &'static str = include_str!("defines.lua");
    let defines = lua.load(DEFINES).set_name("defines").eval()?;

    Ok(defines)
}
