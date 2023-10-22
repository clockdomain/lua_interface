use rlua::{Lua, Table};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub rootdir: String,
    pub debug: bool,
}

pub fn from_lua(lua_code: &str) -> Result<Config, rlua::Error> {
    // Create a Lua context.
    let lua = Lua::new();

    // Load and execute the Lua code.
    let rust_config = lua.context(|lua_ctx| {
        lua_ctx.load(lua_code).exec().map(|_| {
            // Get the Config table
            let config_table: Table = lua_ctx.globals().get("Config").unwrap();

            // Extract values from the table and construct the Config struct
            Config {
                rootdir: config_table.get::<_, String>("rootdir").unwrap(),
                debug: config_table.get::<_, bool>("debug").unwrap(),
            }
        })
    });

    rust_config
}
