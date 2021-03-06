use std::{
    fs::File,
    io::{BufReader, Read},
    path::Path,
};

use mlua::{Lua, Table, Value};

use crate::error::Error;


pub fn run_code<C: AsRef<[u8]>, N: AsRef<str>>(lua: &mut Lua, code: C, name: N) -> Result<(), Error> {
    lua.load(code.as_ref()).set_name(name.as_ref())?.exec()?;
    Ok(())
}

pub fn run_file<P: AsRef<Path>>(lua: &mut Lua, path: P) -> Result<(), Error> {
    let path = path.as_ref();
    let name = path.as_os_str().to_str().unwrap();

    log::debug!("Running Lua: {}", path.display());

    let mut reader = BufReader::new(File::open(path)?);
    let mut source = vec![];
    reader.read_to_end(&mut source)?;

    run_code(lua, source, name)
}

pub fn set_mod_path<P: AsRef<Path>>(lua: &mut Lua, mod_path: P) -> Result<(), Error> {
    let preload = format!(
        r#"

            package.path = "./data/core/lualib/?.lua;./data/libs/?.lua;{}/?.lua"
            print("Set package.path = '" .. package.path .. "'")

        "#,
        mod_path.as_ref().display()
    );
    run_code(lua, &preload, "set-mod-path")
}

pub fn get_data_raw(lua: &Lua) -> Result<mlua::Value, Error> {
    Ok(lua.globals().get::<_, Table>("data")?.get::<_, Value>("raw")?)
}
