use mlua::prelude::*;
use crate::error::SpanreedError;
use crate::config::{find_config};

/// Creates a lua object with data from a file loaded
pub fn get_lua_config(name: &str) -> Result<Lua, SpanreedError> {
    let lua = Lua::new();
    let function_data = get_functions_file_data(name)?;
    lua.load(&function_data).exec()?;
    return Ok(lua);
}

/// Returns the contents of the lua file
fn get_functions_file_data(name: &str) -> Result<Vec<u8>, SpanreedError> {
    let function_path = match find_config(name, "lua") {
        Ok(path) => path,
        Err(e) => {
            if name == "functions" {
                return Ok(Vec::new());
            } else {
                return Err(e);
            }
        }
    };
    return Ok(std::fs::read(function_path)?);
}
