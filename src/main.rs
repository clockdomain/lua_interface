use rlua::{Lua, Table, Value};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub rootdir: String,
    pub debug: bool,
}

fn main() {
    // Create a Lua context.
    let lua = Lua::new();

    // Load and execute the Lua config file.
    lua.context(|lua_ctx| {
        lua_ctx
            .load(
                r#"
            Config = {
              rootdir = "/abc/123",
              debug = true,
            }
            "#,
            )
            .exec()
            .unwrap();
        // Get the Config table
        let config_table: Table = lua_ctx.globals().get("Config").unwrap();

        // Extract values from the table and construct the Config struct
        let rust_config = Config {
            rootdir: config_table.get::<_, String>("rootdir").unwrap(),
            debug: config_table.get::<_, bool>("debug").unwrap(),
        };

        // Use the Rust Config struct
        println!("{:?}", rust_config);
    });
}
